const EMOJI: &str = "🪣  ";

use saucer::{prelude::*, Fs, Utf8PathBuf};

#[derive(Default, Debug, Clone, Parser)]
pub(crate) struct BucketCommand {
    #[clap(flatten)]
    pub(crate) opts: BucketOpts,
}

#[derive(Default, Debug, Clone, Parser)]
pub(crate) struct BucketOpts {
    #[clap(long, default_value_t = crate::relative_dir("awc-web/src/server/public"))]
    public_dir: Utf8PathBuf,

    #[clap(long, default_value_t = crate::relative_dir("awc-web/src/browser/bucket"))]
    bucket_dir: Utf8PathBuf,
}

impl Saucer for BucketCommand {
    /// Copies everything in the bucket to /public
    fn beam(&self) -> Result<()> {
        Fs::copy_dir_all(&self.opts.bucket_dir, &self.opts.public_dir, EMOJI)
    }

    fn prefix(&self) -> String {
        EMOJI.to_string()
    }

    fn description(&self) -> String {
        "bucket copy".to_string()
    }
}
