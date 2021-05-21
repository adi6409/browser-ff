//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgIEncoder.idl
//


/// `interface imgIEncoder : nsIAsyncInputStream`
///

/// ```text
/// /**
///  * imgIEncoder interface
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct imgIEncoder {
    vtable: *const imgIEncoderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for imgIEncoder.
unsafe impl XpCom for imgIEncoder {
    const IID: nsIID = nsID(0x4baa2d6e, 0xfee7, 0x42df,
        [0xae, 0x3f, 0x5f, 0xbe, 0xbc, 0x0c, 0x26, 0x7c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for imgIEncoder {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from imgIEncoder.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait imgIEncoderCoerce {
    /// Cheaply cast a value of this type from a `imgIEncoder`.
    fn coerce_from(v: &imgIEncoder) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl imgIEncoderCoerce for imgIEncoder {
    #[inline]
    fn coerce_from(v: &imgIEncoder) -> &Self {
        v
    }
}

impl imgIEncoder {
    /// Cast this `imgIEncoder` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: imgIEncoderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for imgIEncoder {
    type Target = nsIAsyncInputStream;
    #[inline]
    fn deref(&self) -> &nsIAsyncInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIAsyncInputStreamCoerce> imgIEncoderCoerce for T {
    #[inline]
    fn coerce_from(v: &imgIEncoder) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every imgIEncoder
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct imgIEncoderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIAsyncInputStreamVTable,

    /* void initFromData ([array, size_is (length), const] in uint8_t data, in unsigned long length, in uint32_t width, in uint32_t height, in uint32_t stride, in uint32_t inputFormat, in AString outputOptions); */
    pub InitFromData: unsafe extern "system" fn (this: *const imgIEncoder, data: *const uint8_t, length: u32, width: uint32_t, height: uint32_t, stride: uint32_t, inputFormat: uint32_t, outputOptions: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void startImageEncode (in uint32_t width, in uint32_t height, in uint32_t inputFormat, in AString outputOptions); */
    pub StartImageEncode: unsafe extern "system" fn (this: *const imgIEncoder, width: uint32_t, height: uint32_t, inputFormat: uint32_t, outputOptions: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void addImageFrame ([array, size_is (length), const] in uint8_t data, in unsigned long length, in uint32_t width, in uint32_t height, in uint32_t stride, in uint32_t frameFormat, in AString frameOptions); */
    pub AddImageFrame: unsafe extern "system" fn (this: *const imgIEncoder, data: *const uint8_t, length: u32, width: uint32_t, height: uint32_t, stride: uint32_t, frameFormat: uint32_t, frameOptions: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void endImageEncode (); */
    pub EndImageEncode: unsafe extern "system" fn (this: *const imgIEncoder) -> ::nserror::nsresult,

    /* [noscript] unsigned long getImageBufferUsed (); */
    pub GetImageBufferUsed: unsafe extern "system" fn (this: *const imgIEncoder, _retval: *mut u32) -> ::nserror::nsresult,

    /* [noscript] charPtr getImageBuffer (); */
    /// Unable to generate binding because `native type char unsupported`
    pub GetImageBuffer: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl imgIEncoder {

    pub const INPUT_FORMAT_RGB: i64 = 0;


    pub const INPUT_FORMAT_RGBA: i64 = 1;


    pub const INPUT_FORMAT_HOSTARGB: i64 = 2;


    /// `void initFromData ([array, size_is (length), const] in uint8_t data, in unsigned long length, in uint32_t width, in uint32_t height, in uint32_t stride, in uint32_t inputFormat, in AString outputOptions);`
    #[inline]
    pub unsafe fn InitFromData(&self, data: *const uint8_t, length: u32, width: uint32_t, height: uint32_t, stride: uint32_t, inputFormat: uint32_t, outputOptions: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).InitFromData)(self, data, length, width, height, stride, inputFormat, outputOptions)
    }



    /// `void startImageEncode (in uint32_t width, in uint32_t height, in uint32_t inputFormat, in AString outputOptions);`
    #[inline]
    pub unsafe fn StartImageEncode(&self, width: uint32_t, height: uint32_t, inputFormat: uint32_t, outputOptions: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).StartImageEncode)(self, width, height, inputFormat, outputOptions)
    }



    /// `void addImageFrame ([array, size_is (length), const] in uint8_t data, in unsigned long length, in uint32_t width, in uint32_t height, in uint32_t stride, in uint32_t frameFormat, in AString frameOptions);`
    #[inline]
    pub unsafe fn AddImageFrame(&self, data: *const uint8_t, length: u32, width: uint32_t, height: uint32_t, stride: uint32_t, frameFormat: uint32_t, frameOptions: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AddImageFrame)(self, data, length, width, height, stride, frameFormat, frameOptions)
    }



    /// `void endImageEncode ();`
    #[inline]
    pub unsafe fn EndImageEncode(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EndImageEncode)(self, )
    }



    /// `[noscript] unsigned long getImageBufferUsed ();`
    #[inline]
    pub unsafe fn GetImageBufferUsed(&self, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetImageBufferUsed)(self, _retval)
    }



    /// `[noscript] charPtr getImageBuffer ();`
    const _GetImageBuffer: () = ();

}


