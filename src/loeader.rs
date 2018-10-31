//! Example of integrating ggez types with the `warmy` resource loader.

use std::path;

use error::*;
use failure::{self, Fail};
use ggez::{self, audio, graphics};
use warmy;
use warmy::SimpleKey;

/// Warmy hands our `load()` method an absolute path, while ggez takes absolute
/// paths into its VFS directory.  Warmy needs to know the real absolute path so
/// it can watch for reloads, so this function strips the path prefix of the warmy
/// Store's root off of the given absolute path and turns it into the style of path
/// that ggez expects.
///
/// Because of this, ggez will have several places that resources *may* live but
/// warmy will only watch for reloads in one of them.  However, that isn't a huge
/// problem: for development you generally want all your assets in one place to
/// begin with, and for deployment you don't need the hotloading functionality.
///
/// TODO: With warmy 0.7 this should not be necessary, figure it out.
fn warmy_to_ggez_path(path: &path::Path, root: &path::Path) -> path::PathBuf {
    let stripped_path = path
        .strip_prefix(root)
        .expect("warmy path is outside of the warmy store?  Should never happen.");
    path::Path::new("/").join(stripped_path)
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct FSKey(path::PathBuf);

// impl Key for FSKey {
//     fn prepare_key(root: &Path)
// }

/// Just a test asset that does nothing.
#[derive(Debug, Copy, Clone)]
pub struct TestAsset;

impl<C> warmy::Load<C, SimpleKey> for TestAsset {
    type Error = failure::Compat<GgezError>;
    fn load(
        key: SimpleKey,
        _store: &mut warmy::Storage<C, SimpleKey>,
        _ctx: &mut C,
    ) -> Result<warmy::Loaded<Self, SimpleKey>, Self::Error> {
        debug!("Loading test asset: {:?}", key);
        Ok(TestAsset.into())
    }
}

/// A wrapper for a ggez Image, so we can implement warmy's `Load` trait on it.
#[derive(Debug, Clone)]
pub struct Image(pub graphics::Image);
impl warmy::Load<ggez::Context, SimpleKey> for Image {
    type Error = failure::Compat<GgezError>;
    fn load(
        key: SimpleKey,
        store: &mut warmy::Storage<ggez::Context, SimpleKey>,
        ctx: &mut ggez::Context,
    ) -> Result<warmy::Loaded<Self, SimpleKey>, Self::Error> {
        if let SimpleKey::Path(path_buf) = key {
            // println!("key: {:?}, path: {:?}", path_buf, store.root());
            let path = warmy_to_ggez_path(path_buf.as_path(), store.root());
            // debug!("Loading image {:?} from file {:?}", path, key.as_path());
            graphics::Image::new(ctx, path)
                .map(|x| warmy::Loaded::from(Image(x)))
                .map_err(|e| GgezError::from(e).compat())
        } else {
            unreachable!()
        }
    }
}

/// A wrapper for a ggez SoundData, so we can implement warmy's `Load` trait on it.
#[derive(Debug, Clone)]
pub struct SoundData(pub audio::SoundData);
impl warmy::Load<ggez::Context, SimpleKey> for SoundData {
    type Error = failure::Compat<GgezError>;
    fn load(
        key: SimpleKey,
        _store: &mut warmy::Storage<ggez::Context, SimpleKey>,
        ctx: &mut ggez::Context,
    ) -> Result<warmy::Loaded<Self, SimpleKey>, Self::Error> {
        if let SimpleKey::Path(path_buf) = key {
            // let path = warmy_to_ggez_path(key.as_path(), store.root());
            // debug!("Loading sound {:?} from file {:?}", path, key.as_path());

            audio::SoundData::new(ctx, path_buf.as_path())
                .map(|x| warmy::Loaded::from(SoundData(x)))
                .map_err(|e| GgezError::from(e).compat())
        } else {
            unreachable!()
        }
    }
}

/// A wrapper for a ggez Font, so we can implement warmy's `Load` trait on it.
///
/// Currently it just forces the font size to 12 pt; we should implement a specific
/// key type for it that includes a font size.
#[derive(Debug, Clone)]
pub struct Font(pub graphics::Font);
impl warmy::Load<ggez::Context, SimpleKey> for Font {
    type Error = failure::Compat<GgezError>;
    fn load(
        key: SimpleKey,
        _store: &mut warmy::Storage<ggez::Context, SimpleKey>,
        ctx: &mut ggez::Context,
    ) -> Result<warmy::Loaded<Self, SimpleKey>, Self::Error> {
        if let SimpleKey::Path(path_buf) = key {
            // let path = warmy_to_ggez_path(key.as_path(), store.root());
            // debug!("Loading font {:?} from file {:?}", path, key.as_path());

            graphics::Font::new(ctx, path_buf.as_path(), 12)
                .map(|x| warmy::Loaded::from(Font(x)))
                .map_err(|e| GgezError::from(e).compat())
        } else {
            unreachable!()
        }
    }
}
