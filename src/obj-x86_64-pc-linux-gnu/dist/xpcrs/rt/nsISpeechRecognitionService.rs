//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/webspeech/recognition/nsISpeechRecognitionService.idl
//


/// `interface nsISpeechGrammarCompilationCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISpeechGrammarCompilationCallback {
    vtable: *const nsISpeechGrammarCompilationCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISpeechGrammarCompilationCallback.
unsafe impl XpCom for nsISpeechGrammarCompilationCallback {
    const IID: nsIID = nsID(0x6fcb6ee8, 0xa6db, 0x49ba,
        [0x9f, 0x06, 0x35, 0x5d, 0x7e, 0xe1, 0x8e, 0xa7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISpeechGrammarCompilationCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISpeechGrammarCompilationCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISpeechGrammarCompilationCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsISpeechGrammarCompilationCallback`.
    fn coerce_from(v: &nsISpeechGrammarCompilationCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISpeechGrammarCompilationCallbackCoerce for nsISpeechGrammarCompilationCallback {
    #[inline]
    fn coerce_from(v: &nsISpeechGrammarCompilationCallback) -> &Self {
        v
    }
}

impl nsISpeechGrammarCompilationCallback {
    /// Cast this `nsISpeechGrammarCompilationCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISpeechGrammarCompilationCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISpeechGrammarCompilationCallback {
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
impl<T: nsISupportsCoerce> nsISpeechGrammarCompilationCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeechGrammarCompilationCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISpeechGrammarCompilationCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISpeechGrammarCompilationCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void grammarCompilationEnd (in SpeechGrammarPtr grammarObject, in boolean success); */
    /// Unable to generate binding because `native type mozilla::dom::SpeechGrammar unsupported`
    pub GrammarCompilationEnd: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISpeechGrammarCompilationCallback {


    /// `void grammarCompilationEnd (in SpeechGrammarPtr grammarObject, in boolean success);`
    const _GrammarCompilationEnd: () = ();

}


/// `interface nsISpeechRecognitionService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISpeechRecognitionService {
    vtable: *const nsISpeechRecognitionServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISpeechRecognitionService.
unsafe impl XpCom for nsISpeechRecognitionService {
    const IID: nsIID = nsID(0x8e97f287, 0xf322, 0x44e8,
        [0x88, 0x88, 0x83, 0x44, 0xfa, 0x40, 0x8e, 0xf8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISpeechRecognitionService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISpeechRecognitionService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISpeechRecognitionServiceCoerce {
    /// Cheaply cast a value of this type from a `nsISpeechRecognitionService`.
    fn coerce_from(v: &nsISpeechRecognitionService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISpeechRecognitionServiceCoerce for nsISpeechRecognitionService {
    #[inline]
    fn coerce_from(v: &nsISpeechRecognitionService) -> &Self {
        v
    }
}

impl nsISpeechRecognitionService {
    /// Cast this `nsISpeechRecognitionService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISpeechRecognitionServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISpeechRecognitionService {
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
impl<T: nsISupportsCoerce> nsISpeechRecognitionServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeechRecognitionService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISpeechRecognitionService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISpeechRecognitionServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void initialize (in SpeechRecognitionWeakPtr aSpeechRecognition); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub Initialize: *const ::libc::c_void,

    /* void processAudioSegment (in AudioSegmentPtr aAudioSegment, in long aSampleRate); */
    /// Unable to generate binding because `native type mozilla::AudioSegment unsupported`
    pub ProcessAudioSegment: *const ::libc::c_void,

    /* void validateAndSetGrammarList (in SpeechGrammarPtr aSpeechGrammar, in nsISpeechGrammarCompilationCallback aCallback); */
    /// Unable to generate binding because `native type mozilla::dom::SpeechGrammar unsupported`
    pub ValidateAndSetGrammarList: *const ::libc::c_void,

    /* void soundEnd (); */
    pub SoundEnd: unsafe extern "system" fn (this: *const nsISpeechRecognitionService) -> ::nserror::nsresult,

    /* void abort (); */
    pub Abort: unsafe extern "system" fn (this: *const nsISpeechRecognitionService) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISpeechRecognitionService {


    /// `void initialize (in SpeechRecognitionWeakPtr aSpeechRecognition);`
    const _Initialize: () = ();


    /// `void processAudioSegment (in AudioSegmentPtr aAudioSegment, in long aSampleRate);`
    const _ProcessAudioSegment: () = ();


    /// `void validateAndSetGrammarList (in SpeechGrammarPtr aSpeechGrammar, in nsISpeechGrammarCompilationCallback aCallback);`
    const _ValidateAndSetGrammarList: () = ();


    /// `void soundEnd ();`
    #[inline]
    pub unsafe fn SoundEnd(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SoundEnd)(self, )
    }



    /// `void abort ();`
    #[inline]
    pub unsafe fn Abort(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Abort)(self, )
    }


}


