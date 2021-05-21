//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/mozITXTToHTMLConv.idl
//


/// `interface mozITXTToHTMLConv : nsIStreamConverter`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozITXTToHTMLConv {
    vtable: *const mozITXTToHTMLConvVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozITXTToHTMLConv.
unsafe impl XpCom for mozITXTToHTMLConv {
    const IID: nsIID = nsID(0x77c0e42a, 0x1dd2, 0x11b2,
        [0x8e, 0xbf, 0xed, 0xc6, 0x60, 0x6f, 0x2f, 0x4b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozITXTToHTMLConv {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozITXTToHTMLConv.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozITXTToHTMLConvCoerce {
    /// Cheaply cast a value of this type from a `mozITXTToHTMLConv`.
    fn coerce_from(v: &mozITXTToHTMLConv) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozITXTToHTMLConvCoerce for mozITXTToHTMLConv {
    #[inline]
    fn coerce_from(v: &mozITXTToHTMLConv) -> &Self {
        v
    }
}

impl mozITXTToHTMLConv {
    /// Cast this `mozITXTToHTMLConv` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozITXTToHTMLConvCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozITXTToHTMLConv {
    type Target = nsIStreamConverter;
    #[inline]
    fn deref(&self) -> &nsIStreamConverter {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIStreamConverterCoerce> mozITXTToHTMLConvCoerce for T {
    #[inline]
    fn coerce_from(v: &mozITXTToHTMLConv) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozITXTToHTMLConv
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozITXTToHTMLConvVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIStreamConverterVTable,

    /* AString scanTXT (in AString text, in unsigned long whattodo); */
    pub ScanTXT: unsafe extern "system" fn (this: *const mozITXTToHTMLConv, text: *const ::nsstring::nsAString, whattodo: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString scanHTML (in AString text, in unsigned long whattodo); */
    pub ScanHTML: unsafe extern "system" fn (this: *const mozITXTToHTMLConv, text: *const ::nsstring::nsAString, whattodo: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* unsigned long citeLevelTXT (in wstring line, out unsigned long logLineStart); */
    pub CiteLevelTXT: unsafe extern "system" fn (this: *const mozITXTToHTMLConv, line: *const i16, logLineStart: *mut u32, _retval: *mut u32) -> ::nserror::nsresult,

    /* void findURLInPlaintext (in wstring text, in long aLength, in long aPos, out long aStartPos, out long aEndPos); */
    pub FindURLInPlaintext: unsafe extern "system" fn (this: *const mozITXTToHTMLConv, text: *const i16, aLength: i32, aPos: i32, aStartPos: *mut i32, aEndPos: *mut i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozITXTToHTMLConv {

    pub const kEntities: i64 = 0;


    pub const kURLs: i64 = 2;


    pub const kGlyphSubstitution: i64 = 4;


    pub const kStructPhrase: i64 = 8;

    /// ```text
    /// /**
    ///   @param text: plain text to scan. May be a line, paragraph (recommended)
    ///                or just a substring.<p>
    ///                Must be non-escaped, pure unicode.<p>
    ///                <em>Note:</em> ScanTXT(a, o) + ScanTXT(b, o) may be !=
    ///                Scan(a + b, o)
    ///   @param whattodo: Bitfield describing the modes of operation
    ///   @result      "<", ">" and "&" are escaped and HTML tags are inserted where
    ///                appropriate.
    ///  */
    /// ```
    ///

    /// `AString scanTXT (in AString text, in unsigned long whattodo);`
    #[inline]
    pub unsafe fn ScanTXT(&self, text: *const ::nsstring::nsAString, whattodo: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ScanTXT)(self, text, whattodo, _retval)
    }


    /// ```text
    /// /**
    ///   Adds additional formatting to user edited text, that the user was too lazy
    ///   or "unknowledged" (DELETEME: is that a word?) to make.
    ///   <p>
    ///   <em>Note:</em> Don't use kGlyphSubstitution with this function. This option
    ///   generates tags, that are unuseable for UAs other than Mozilla. This would
    ///   be a data loss bug.
    ///
    ///   @param text: HTML source to scan. May be a line, paragraph (recommended)
    ///                or just a substring.<p>
    ///                Must be correct HTML. "<", ">" and "&" must be escaped,
    ///                other chars must be pure unicode.<p>
    ///                <em>Note:</em> ScanTXT(a, o) + ScanTXT(b, o) may be !=
    ///                Scan(a + b, o)
    ///   @param whattodo: Bitfield describing the modes of operation
    ///   @result      Additional HTML tags are inserted where appropriate.
    ///  */
    /// ```
    ///

    /// `AString scanHTML (in AString text, in unsigned long whattodo);`
    #[inline]
    pub unsafe fn ScanHTML(&self, text: *const ::nsstring::nsAString, whattodo: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ScanHTML)(self, text, whattodo, _retval)
    }


    /// ```text
    /// /**
    ///   @param line: line in original msg, possibly starting starting with
    ///                txt quote tags like ">"
    ///   @param logLineStart: pos in line, where the real content (logical line)
    ///                begins, i.e. pos after all txt quote tags.
    ///                E.g. position of "t" in "> > text".
    ///                Initial value must be 0, unless line is not real line.
    ///   @return      Cite Level, i.e. number of txt quote tags found, i.e. number of
    ///                nested quotes.
    ///  */
    /// ```
    ///

    /// `unsigned long citeLevelTXT (in wstring line, out unsigned long logLineStart);`
    #[inline]
    pub unsafe fn CiteLevelTXT(&self, line: *const i16, logLineStart: *mut u32, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).CiteLevelTXT)(self, line, logLineStart, _retval)
    }


    /// ```text
    /// /**
    ///  @param a wide string to scan for the presence of a URL.
    ///  @param aLength --> the length of the buffer to be scanned
    ///  @param aPos --> the position in the buffer to start scanning for a url
    ///
    ///  aStartPos --> index into the start of a url (-1 if no url found)
    ///  aEndPos --> index of the last character in the url (-1 if no url found)
    ///  */
    /// ```
    ///

    /// `void findURLInPlaintext (in wstring text, in long aLength, in long aPos, out long aStartPos, out long aEndPos);`
    #[inline]
    pub unsafe fn FindURLInPlaintext(&self, text: *const i16, aLength: i32, aPos: i32, aStartPos: *mut i32, aEndPos: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).FindURLInPlaintext)(self, text, aLength, aPos, aStartPos, aEndPos)
    }


}


