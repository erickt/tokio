use crate::io::util::write_all::{write_all, WriteAll};
use crate::io::AsyncWrite;

pub trait AsyncWriteExt: AsyncWrite {
        /// Attempts to write an entire buffer into this writer.
        ///
        /// Equivalent to:
        ///
        /// ```ignore
        /// async fn write_all(&mut self, buf: &[u8]) -> io::Result<()>;
        /// ```
        ///
        /// This method will continuously call [`write`] until there is no more data
        /// to be written. This method will not return until the entire buffer
        /// has been successfully written or such an error occurs. The first
        /// error generated from this method will be returned.
        ///
        /// # Errors
        ///
        /// This function will return the first error that [`write`] returns.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use tokio::io::{self, AsyncWriteExt};
        /// use tokio::fs::File;
        ///
        /// #[tokio::main]
        /// async fn main() -> io::Result<()> {
        ///     let mut buffer = File::create("foo.txt").await?;
        ///
        ///     buffer.write_all(b"some bytes").await?;
        ///     Ok(())
        /// }
        /// ```
        ///
        /// [`write`]: AsyncWriteExt::write
        fn write_all<'a>(&'a mut self, src: &'a [u8]) -> WriteAll<'a, Self>
        where
            Self: Unpin,
        {
            write_all(self, src)
        }
}

impl<W: AsyncWrite + ?Sized> AsyncWriteExt for W {}
