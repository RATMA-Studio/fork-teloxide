use crate::{
    payloads,
    requests::Payload,
    types::{InputFile, InputFileLike, InputMedia, InputPaidMedia, InputSticker},
};

// Used in the `SendPoll` impl below.
use crate::types::{InputPollMedia, InputPollOptionMedia};

/// Payloads that need to be sent as `multipart/form-data` because they contain
/// files inside.
pub trait MultipartPayload: Payload {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile));

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile));
}

impl MultipartPayload for payloads::SendPaidMedia {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter().flat_map(InputPaidMedia::files).for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter_mut().flat_map(InputPaidMedia::files_mut).for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::SendMediaGroup {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter().flat_map(InputMedia::files).for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter_mut().flat_map(InputMedia::files_mut).for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::EditMessageMedia {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.files().for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.files_mut().for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::EditMessageMediaInline {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.files().for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.files_mut().for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::SetMyProfilePhoto {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.photo.files().for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.photo.files_mut().for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::SendPoll {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        if let Some(media) = self.media.as_ref() {
            InputPollMedia::files(media).for_each(|f| f.copy_into(into));
        }
        if let Some(explanation_media) = self.explanation_media.as_ref() {
            InputPollMedia::files(explanation_media).for_each(|f| f.copy_into(into));
        }
        for option in &self.options {
            if let Some(option_media) = option.media.as_ref() {
                InputPollOptionMedia::files(option_media).for_each(|f| f.copy_into(into));
            }
        }
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        if let Some(media) = self.media.as_mut() {
            InputPollMedia::files_mut(media).for_each(|f| f.move_into(into));
        }
        if let Some(explanation_media) = self.explanation_media.as_mut() {
            InputPollMedia::files_mut(explanation_media).for_each(|f| f.move_into(into));
        }
        for option in &mut self.options {
            if let Some(option_media) = option.media.as_mut() {
                InputPollOptionMedia::files_mut(option_media).for_each(|f| f.move_into(into));
            }
        }
    }
}

impl MultipartPayload for payloads::CreateNewStickerSet {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.stickers
            .iter()
            .for_each(|InputSticker { sticker: f, .. }: &InputSticker| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.stickers
            .iter_mut()
            .for_each(|InputSticker { sticker: f, .. }: &mut InputSticker| f.move_into(into))
    }
}
