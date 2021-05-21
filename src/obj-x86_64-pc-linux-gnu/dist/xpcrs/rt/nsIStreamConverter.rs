//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsIStreamConverter.idl
//


/// `interface nsIStreamConverter : nsIStreamListener`
///

/// ```text
/// /**
///  * nsIStreamConverter provides an interface to implement when you have code
///  * that converts data from one type to another.
///  *
///  * Suppose you had code that converted plain text into HTML. You could implement
///  * this interface to allow everyone else to use your conversion logic using a
///  * standard api.
///  * <p>
///  * <b>STREAM CONVERTER USERS</b>
///  *
///  * There are currently two ways to use a stream converter:
///  * <ol>
///  * <li> <b>SYNCHRONOUS</b> Stream to Stream
///  *    You can supply the service with a stream of type X
///  *    and it will convert it to your desired output type and return
///  *    a converted (blocking) stream to you.</li>
///  *
///  * <li> <b>ASYNCHRONOUS</b> nsIStreamListener to nsIStreamListener
///  *    You can supply data directly to the converter by calling it's
///  *    nsIStreamListener::OnDataAvailable() method. It will then
///  *    convert that data from type X to your desired output type and
///  *    return converted data to you via the nsIStreamListener you passed
///  *    in by calling its OnDataAvailable() method.</li>
///  * </ol>
///  * <p>
///  *
///  * <b>STREAM CONVERTER SUPPLIERS</b>
///  *
///  * Registering a stream converter:
///  * Stream converter registration is a two step process. First of all the stream
///  * converter implementation must register itself with the component manager using
///  * a contractid in the format below. Second, the stream converter must add the contractid
///  * to the registry.
///  *
///  * Stream converter contractid format (the stream converter root key is defined in this
    ///  * file):
///  *
///  * <pre>@mozilla.org/streamconv;1?from=FROM_MIME_TYPE&to=TO_MIME_TYPE</pre>
///  *
///  * @author Jud Valeski
///  * @see nsIStreamConverterService
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStreamConverter {
    vtable: *const nsIStreamConverterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStreamConverter.
unsafe impl XpCom for nsIStreamConverter {
    const IID: nsIID = nsID(0x0b6e2c69, 0x5cf5, 0x48b0,
        [0x9d, 0xfd, 0xc9, 0x59, 0x50, 0xe2, 0xcc, 0x7b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStreamConverter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStreamConverter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStreamConverterCoerce {
    /// Cheaply cast a value of this type from a `nsIStreamConverter`.
    fn coerce_from(v: &nsIStreamConverter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStreamConverterCoerce for nsIStreamConverter {
    #[inline]
    fn coerce_from(v: &nsIStreamConverter) -> &Self {
        v
    }
}

impl nsIStreamConverter {
    /// Cast this `nsIStreamConverter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStreamConverterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStreamConverter {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIStreamListenerCoerce> nsIStreamConverterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamConverter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStreamConverter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStreamConverterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIStreamListenerVTable,

    /* nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aCtxt); */
    pub Convert: unsafe extern "system" fn (this: *const nsIStreamConverter, aFromStream: *const nsIInputStream, aFromType: *const libc::c_char, aToType: *const libc::c_char, aCtxt: *const nsISupports, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* void asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aCtxt); */
    pub AsyncConvertData: unsafe extern "system" fn (this: *const nsIStreamConverter, aFromType: *const libc::c_char, aToType: *const libc::c_char, aListener: *const nsIStreamListener, aCtxt: *const nsISupports) -> ::nserror::nsresult,

    /* ACString getConvertedType (in ACString aFromType, in nsIChannel aChannel); */
    pub GetConvertedType: unsafe extern "system" fn (this: *const nsIStreamConverter, aFromType: *const ::nsstring::nsACString, aChannel: *const nsIChannel, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStreamConverter {

    /// ```text
    /// /**
    ///      * <b>SYNCRONOUS VERSION</b>
    ///      * Converts a stream of one type, to a stream of another type.
    ///      *
    ///      * Use this method when you have a stream you want to convert.
    ///      *
    ///      * @param aFromStream   The stream representing the original/raw data.
    ///      * @param aFromType     The MIME type of aFromStream.
    ///      * @param aToType       The MIME type of the returned stream.
    ///      * @param aCtxt         Either an opaque context, or a converter specific context
    ///      *                      (implementation specific).
    ///      * @return              The converted stream. NOTE: The returned stream may not
    ///      *                      already be converted. An efficient stream converter
    ///      *                      implementation will converter data on demand rather than
    ///      *                      buffering the converted data until it is used.
    ///      */
    /// ```
    ///

    /// `nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aCtxt);`
    #[inline]
    pub unsafe fn Convert(&self, aFromStream: *const nsIInputStream, aFromType: *const libc::c_char, aToType: *const libc::c_char, aCtxt: *const nsISupports, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).Convert)(self, aFromStream, aFromType, aToType, aCtxt, _retval)
    }


    /// ```text
    /// /**
    ///      * <b>ASYNCRONOUS VERSION</b>
    ///      * Converts data arriving via the converter's nsIStreamListener::OnDataAvailable()
    ///      * method from one type to another, pushing the converted data out to the caller
    ///      * via aListener::OnDataAvailable().
    ///      *
    ///      * Use this method when you want to proxy (and convert) nsIStreamListener callbacks
    ///      * asynchronously.
    ///      *
    ///      * @param aFromType     The MIME type of the original/raw data.
    ///      * @param aToType       The MIME type of the converted data.
    ///      * @param aListener     The listener who receives the converted data.
    ///      * @param aCtxt         Either an opaque context, or a converter specific context
    ///      *                      (implementation specific).
    ///      */
    /// ```
    ///

    /// `void asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aCtxt);`
    #[inline]
    pub unsafe fn AsyncConvertData(&self, aFromType: *const libc::c_char, aToType: *const libc::c_char, aListener: *const nsIStreamListener, aCtxt: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).AsyncConvertData)(self, aFromType, aToType, aListener, aCtxt)
    }


    /// ```text
    /// /**
    ///      * Returns the content type that the stream listener passed to asyncConvertData will
    ///      * see on the channel if the conversion is being done from aFromType to * /*.
    ///      *
    ///      * @param aFromType     The type of the content prior to conversion.
    ///      * @param aChannel      The channel that we'd like to convert. May be null.
    ///      *
    ///      * @throws if the converter does not support conversion to * /* or if it doesn't know
    ///      *         the type in advance.
    ///      */
    /// ```
    ///

    /// `ACString getConvertedType (in ACString aFromType, in nsIChannel aChannel);`
    #[inline]
    pub unsafe fn GetConvertedType(&self, aFromType: *const ::nsstring::nsACString, aChannel: *const nsIChannel, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetConvertedType)(self, aFromType, aChannel, _retval)
    }


}


