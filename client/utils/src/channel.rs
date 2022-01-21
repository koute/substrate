use futures::{channel::mpsc::TrySendError, Stream};
use std::{
	pin::Pin,
	sync::{Arc, Weak},
	task::Poll,
};

pub trait SubscriptionRegistry: Send + Sync {
	fn unsubscribe(&self, id: u64, was_triggered: bool);
}

pub struct TracingUnboundedSender<T> {
	tx: crate::mpsc::TracingUnboundedSender<T>,
}

impl<T> TracingUnboundedSender<T> {
	pub fn unbounded_send(&self, payload: T) -> Result<(), TrySendError<T>> {
		self.tx.unbounded_send(payload)
	}
}

impl<T> std::fmt::Debug for TracingUnboundedSender<T> {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		todo!();
	}
}

pub struct TracingUnboundedReceiver<T> {
	rx: crate::mpsc::TracingUnboundedReceiver<T>,
	id: u64,
	registry: Option<Weak<dyn SubscriptionRegistry>>,
	was_triggered: bool,
}

impl<T> Stream for TracingUnboundedReceiver<T> {
	type Item = <crate::mpsc::TracingUnboundedReceiver<T> as Stream>::Item;
	fn poll_next(
		mut self: Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> Poll<Option<Self::Item>> {
		let result = Stream::poll_next(Pin::new(&mut self.rx), cx);
		if result.is_ready() {
			self.was_triggered = true;
		}
		result
	}
}

pub fn tracing_unbounded<R, T>(
	tracing_key: &'static str,
	registry: Arc<R>,
	id: u64,
) -> (TracingUnboundedSender<T>, TracingUnboundedReceiver<T>)
where
	R: SubscriptionRegistry + 'static,
{
	let registry = Arc::downgrade(&(registry as Arc<dyn SubscriptionRegistry>));
	let (tx, rx) = crate::mpsc::tracing_unbounded(tracing_key);

	let tx = TracingUnboundedSender { tx };
	let rx = TracingUnboundedReceiver { rx, id, registry: Some(registry), was_triggered: false };

	(tx, rx)
}

impl<T> Drop for TracingUnboundedReceiver<T> {
	fn drop(&mut self) {
		if let Some(registry) = self.registry.take().unwrap().upgrade() {
			registry.unsubscribe(self.id, self.was_triggered);
		}
	}
}
