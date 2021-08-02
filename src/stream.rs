/*
 * Created on Wed Jul 28 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{
    io::{self, Read, Write},
    pin::Pin,
    task::{Context, Poll},
};

use futures::{AsyncRead, AsyncWrite};

/// Write large data with specific chunk size
#[derive(Debug, Clone)]
pub struct ChunkedWriteStream<S> {
    stream: S,
    max_size: usize,
}

impl<S> ChunkedWriteStream<S> {
    pub fn new(stream: S, max_size: usize) -> Self {
        Self { stream, max_size }
    }

    pub fn inner(&self) -> &S {
        &self.stream
    }

    pub fn inner_mut(&mut self) -> &mut S {
        &mut self.stream
    }

    pub fn unwrap(self) -> S {
        self.stream
    }
}

impl<S: Write> Write for ChunkedWriteStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = buf.len().min(self.max_size);

        self.stream.write(&buf[..len])
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}

impl<S: Read> Read for ChunkedWriteStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for ChunkedWriteStream<S> {
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        let len = buf.len().min(self.max_size);

        Pin::new(&mut self.stream).poll_write(cx, &buf[..len])
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_close(cx)
    }
}

impl<S: AsyncRead + Unpin> AsyncRead for ChunkedWriteStream<S> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.stream).poll_read(cx, buf)
    }
}
