//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsIStreamConverterService.idl
//


/// `interface nsIStreamConverterService : nsISupports`
///

/// ```text
/// /**
///  * The nsIStreamConverterService is a higher level stream converter factory
///  * responsible for locating and creating stream converters
///  * (nsIStreamConverter).
///  *
///  * This service retrieves an interface that can convert data from a particular
///  * MIME type, to a particular MIME type. It is responsible for any intermediary
///  * conversion required in order to get from X to Z, assuming direct conversion
///  * is not possible.
///  *
///  * @author Jud Valeski
///  * @see nsIStreamConverter
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStreamConverterService {
    vtable: *const nsIStreamConverterServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStreamConverterService.
unsafe impl XpCom for nsIStreamConverterService {
    const IID: nsIID = nsID(0xf2b1ab53, 0xf0bd, 0x4adb,
        [0x93, 0x65, 0xe5, 0x9b, 0x17, 0x01, 0xa2, 0x58]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStreamConverterService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStreamConverterService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStreamConverterServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIStreamConverterService`.
    fn coerce_from(v: &nsIStreamConverterService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStreamConverterServiceCoerce for nsIStreamConverterService {
    #[inline]
    fn coerce_from(v: &nsIStreamConverterService) -> &Self {
        v
    }
}

impl nsIStreamConverterService {
    /// Cast this `nsIStreamConverterService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStreamConverterServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStreamConverterService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIStreamConverterServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamConverterService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStreamConverterService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStreamConverterServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean canConvert (in string aFromType, in string aToType); */
    pub CanConvert: unsafe extern "system" fn (this: *const nsIStreamConverterService, aFromType: *const libc::c_char, aToType: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* ACString convertedType (in ACString aFromType, in nsIChannel aChannel); */
    pub ConvertedType: unsafe extern "system" fn (this: *const nsIStreamConverterService, aFromType: *const ::nsstring::nsACString, aChannel: *const nsIChannel, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aContext); */
    pub Convert: unsafe extern "system" fn (this: *const nsIStreamConverterService, aFromStream: *const nsIInputStream, aFromType: *const libc::c_char, aToType: *const libc::c_char, aContext: *const nsISupports, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* nsIStreamListener asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aContext); */
    pub AsyncConvertData: unsafe extern "system" fn (this: *const nsIStreamConverterService, aFromType: *const libc::c_char, aToType: *const libc::c_char, aListener: *const nsIStreamListener, aContext: *const nsISupports, _retval: *mut*const nsIStreamListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStreamConverterService {

    /// ```text
    /// /**
    ///      * Tests whether conversion between the two specified types is possible.
    ///      * This is cheaper than calling convert()/asyncConvertData(); it is not
    ///      * necessary to call this function before calling one of those, though.
    ///      */
    /// ```
    ///

    /// `boolean canConvert (in string aFromType, in string aToType);`
    #[inline]
    pub unsafe fn CanConvert(&self, aFromType: *const libc::c_char, aToType: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanConvert)(self, aFromType, aToType, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns the content type that will be returned from a converter
    ///      * created with aFromType and  * /*.
    ///      * Can fail if no converters support this conversion, or if the
    ///      * output type isn't known in advance.
    ///      */
    /// ```
    ///

    /// `ACString convertedType (in ACString aFromType, in nsIChannel aChannel);`
    #[inline]
    pub unsafe fn ConvertedType(&self, aFromType: *const ::nsstring::nsACString, aChannel: *const nsIChannel, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ConvertedType)(self, aFromType, aChannel, _retval)
    }


    /// ```text
    /// /**
    ///      * <b>SYNCHRONOUS VERSION</b>
    ///      * Converts a stream of one type, to a stream of another type.
    ///      *
    ///      * Use this method when you have a stream you want to convert.
    ///      *
    ///      * @param aFromStream   The stream representing the original/raw data.
    ///      * @param aFromType     The MIME type of aFromStream.
    ///      * @param aToType       The MIME type of the returned stream.
    ///      * @param aContext      Either an opaque context, or a converter specific
    ///      *                      context (implementation specific).
    ///      * @return              The converted stream. NOTE: The returned stream
    ///      *                      may not already be converted. An efficient stream
    ///      *                      converter implementation will convert data on
    ///      *                      demand rather than buffering the converted data
    ///      *                      until it is used.
    ///      */
    /// ```
    ///

    /// `nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aContext);`
    #[inline]
    pub unsafe fn Convert(&self, aFromStream: *const nsIInputStream, aFromType: *const libc::c_char, aToType: *const libc::c_char, aContext: *const nsISupports, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).Convert)(self, aFromStream, aFromType, aToType, aContext, _retval)
    }


    /// ```text
    /// /**
    ///      * <b>ASYNCHRONOUS VERSION</b>
    ///      * Retrieves a nsIStreamListener that receives the original/raw data via its
    ///      * nsIStreamListener::OnDataAvailable() callback, then converts and pushes
    ///      * the data to aListener.
    ///      *
    ///      * Use this method when you want to proxy (and convert) nsIStreamListener
    ///      * callbacks asynchronously.
    ///      *
    ///      * @param aFromType     The MIME type of the original/raw data.
    ///      * @param aToType       The MIME type of the converted data.
    ///      * @param aListener     The listener that receives the converted data.
    ///      * @param aCtxt         Either an opaque context, or a converter specific
    ///      *                      context (implementation specific).
    ///      * @return              A nsIStreamListener that receives data via its
    ///      *                      OnDataAvailable() method.
    ///      */
    /// ```
    ///

    /// `nsIStreamListener asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aContext);`
    #[inline]
    pub unsafe fn AsyncConvertData(&self, aFromType: *const libc::c_char, aToType: *const libc::c_char, aListener: *const nsIStreamListener, aContext: *const nsISupports, _retval: *mut*const nsIStreamListener) -> ::nserror::nsresult {
        ((*self.vtable).AsyncConvertData)(self, aFromType, aToType, aListener, aContext, _retval)
    }


}


