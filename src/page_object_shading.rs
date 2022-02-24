//! Defines the [PdfPageShadingObject] struct, exposing functionality related to a single
//! page object of type `PdfPageObjectType::Shading`.

use crate::bindgen::FPDF_PAGEOBJECT;
use crate::bindings::PdfiumLibraryBindings;
use crate::page_object::internal::PdfPageObjectPrivate;
use crate::page_objects::PdfPageObjectIndex;

pub struct PdfPageShadingObject<'a> {
    index: PdfPageObjectIndex,
    is_object_memory_owned_by_page: bool,
    handle: FPDF_PAGEOBJECT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageShadingObject<'a> {
    pub(crate) fn from_pdfium(
        index: PdfPageObjectIndex,
        handle: FPDF_PAGEOBJECT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageShadingObject {
            index,
            is_object_memory_owned_by_page: true,
            handle,
            bindings,
        }
    }
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageShadingObject<'a> {
    #[inline]
    fn get_handle(&self) -> &FPDF_PAGEOBJECT {
        &self.handle
    }

    #[inline]
    fn index_impl(&self) -> PdfPageObjectIndex {
        self.index
    }

    #[inline]
    fn is_object_memory_owned_by_page(&self) -> bool {
        self.is_object_memory_owned_by_page
    }

    #[inline]
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}