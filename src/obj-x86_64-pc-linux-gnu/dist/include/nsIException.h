/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIException.idl
 */

#ifndef __gen_nsIException_h__
#define __gen_nsIException_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIStackFrame */
#define NS_ISTACKFRAME_IID_STR "28bfb2a2-5ea6-4738-918b-049dc4d51f0b"

#define NS_ISTACKFRAME_IID \
  {0x28bfb2a2, 0x5ea6, 0x4738, \
    { 0x91, 0x8b, 0x04, 0x9d, 0xc4, 0xd5, 0x1f, 0x0b }}

class NS_NO_VTABLE nsIStackFrame : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTACKFRAME_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStackFrame;

  /* [binaryname(FilenameXPCOM),implicit_jscontext] readonly attribute AString filename; */
  NS_IMETHOD GetFilenameXPCOM(JSContext* cx, nsAString& aFilename) = 0;

  /* [binaryname(NameXPCOM),implicit_jscontext] readonly attribute AString name; */
  NS_IMETHOD GetNameXPCOM(JSContext* cx, nsAString& aName) = 0;

  /* [binaryname(SourceIdXPCOM),implicit_jscontext] readonly attribute int32_t sourceId; */
  NS_IMETHOD GetSourceIdXPCOM(JSContext* cx, int32_t *aSourceId) = 0;

  /* [binaryname(LineNumberXPCOM),implicit_jscontext] readonly attribute int32_t lineNumber; */
  NS_IMETHOD GetLineNumberXPCOM(JSContext* cx, int32_t *aLineNumber) = 0;

  /* [binaryname(ColumnNumberXPCOM),implicit_jscontext] readonly attribute int32_t columnNumber; */
  NS_IMETHOD GetColumnNumberXPCOM(JSContext* cx, int32_t *aColumnNumber) = 0;

  /* readonly attribute AUTF8String sourceLine; */
  NS_IMETHOD GetSourceLine(nsACString& aSourceLine) = 0;

  /* [binaryname(AsyncCauseXPCOM),implicit_jscontext] readonly attribute AString asyncCause; */
  NS_IMETHOD GetAsyncCauseXPCOM(JSContext* cx, nsAString& aAsyncCause) = 0;

  /* [binaryname(AsyncCallerXPCOM),implicit_jscontext] readonly attribute nsIStackFrame asyncCaller; */
  NS_IMETHOD GetAsyncCallerXPCOM(JSContext* cx, nsIStackFrame **aAsyncCaller) = 0;

  /* [binaryname(CallerXPCOM),implicit_jscontext] readonly attribute nsIStackFrame caller; */
  NS_IMETHOD GetCallerXPCOM(JSContext* cx, nsIStackFrame **aCaller) = 0;

  /* [binaryname(FormattedStackXPCOM),implicit_jscontext] readonly attribute AString formattedStack; */
  NS_IMETHOD GetFormattedStackXPCOM(JSContext* cx, nsAString& aFormattedStack) = 0;

  /* readonly attribute jsval nativeSavedFrame; */
  NS_IMETHOD GetNativeSavedFrame(JS::MutableHandleValue aNativeSavedFrame) = 0;

  /* [binaryname(ToStringXPCOM),implicit_jscontext] AUTF8String toString (); */
  NS_IMETHOD ToStringXPCOM(JSContext* cx, nsACString& _retval) = 0;

  /* [nostdcall,notxpcom] void getFilename (in JSContext aCx, out AString aFilename); */
  virtual void GetFilename(JSContext * aCx, nsAString& aFilename) = 0;

  /* [nostdcall,notxpcom] void getName (in JSContext aCx, out AString aName); */
  virtual void GetName(JSContext * aCx, nsAString& aName) = 0;

  /* [nostdcall,notxpcom] int32_t getSourceId (in JSContext aCx); */
  virtual int32_t GetSourceId(JSContext * aCx) = 0;

  /* [nostdcall,notxpcom] int32_t getLineNumber (in JSContext aCx); */
  virtual int32_t GetLineNumber(JSContext * aCx) = 0;

  /* [nostdcall,notxpcom] int32_t getColumnNumber (in JSContext aCx); */
  virtual int32_t GetColumnNumber(JSContext * aCx) = 0;

  /* [nostdcall,notxpcom] void getAsyncCause (in JSContext aCx, out AString aAsyncCause); */
  virtual void GetAsyncCause(JSContext * aCx, nsAString& aAsyncCause) = 0;

  /* [nostdcall,notxpcom] StackFrameRef getAsyncCaller (in JSContext aCx); */
  virtual already_AddRefed<nsIStackFrame> GetAsyncCaller(JSContext * aCx) = 0;

  /* [nostdcall,notxpcom] StackFrameRef getCaller (in JSContext aCx); */
  virtual already_AddRefed<nsIStackFrame> GetCaller(JSContext * aCx) = 0;

  /* [nostdcall,notxpcom] void getFormattedStack (in JSContext aCx, out AString aFormattedStack); */
  virtual void GetFormattedStack(JSContext * aCx, nsAString& aFormattedStack) = 0;

  /* [binaryname(ToString),nostdcall,notxpcom] void toStringInfallible (in JSContext aCx, out AUTF8String aString); */
  virtual void ToString(JSContext * aCx, nsACString& aString) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStackFrame, NS_ISTACKFRAME_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTACKFRAME \
  NS_IMETHOD GetFilenameXPCOM(JSContext* cx, nsAString& aFilename) override; \
  NS_IMETHOD GetNameXPCOM(JSContext* cx, nsAString& aName) override; \
  NS_IMETHOD GetSourceIdXPCOM(JSContext* cx, int32_t *aSourceId) override; \
  NS_IMETHOD GetLineNumberXPCOM(JSContext* cx, int32_t *aLineNumber) override; \
  NS_IMETHOD GetColumnNumberXPCOM(JSContext* cx, int32_t *aColumnNumber) override; \
  NS_IMETHOD GetSourceLine(nsACString& aSourceLine) override; \
  NS_IMETHOD GetAsyncCauseXPCOM(JSContext* cx, nsAString& aAsyncCause) override; \
  NS_IMETHOD GetAsyncCallerXPCOM(JSContext* cx, nsIStackFrame **aAsyncCaller) override; \
  NS_IMETHOD GetCallerXPCOM(JSContext* cx, nsIStackFrame **aCaller) override; \
  NS_IMETHOD GetFormattedStackXPCOM(JSContext* cx, nsAString& aFormattedStack) override; \
  NS_IMETHOD GetNativeSavedFrame(JS::MutableHandleValue aNativeSavedFrame) override; \
  NS_IMETHOD ToStringXPCOM(JSContext* cx, nsACString& _retval) override; \
  virtual void GetFilename(JSContext * aCx, nsAString& aFilename) override; \
  virtual void GetName(JSContext * aCx, nsAString& aName) override; \
  virtual int32_t GetSourceId(JSContext * aCx) override; \
  virtual int32_t GetLineNumber(JSContext * aCx) override; \
  virtual int32_t GetColumnNumber(JSContext * aCx) override; \
  virtual void GetAsyncCause(JSContext * aCx, nsAString& aAsyncCause) override; \
  virtual already_AddRefed<nsIStackFrame> GetAsyncCaller(JSContext * aCx) override; \
  virtual already_AddRefed<nsIStackFrame> GetCaller(JSContext * aCx) override; \
  virtual void GetFormattedStack(JSContext * aCx, nsAString& aFormattedStack) override; \
  virtual void ToString(JSContext * aCx, nsACString& aString) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTACKFRAME \
  nsresult GetFilenameXPCOM(JSContext* cx, nsAString& aFilename); \
  nsresult GetNameXPCOM(JSContext* cx, nsAString& aName); \
  nsresult GetSourceIdXPCOM(JSContext* cx, int32_t *aSourceId); \
  nsresult GetLineNumberXPCOM(JSContext* cx, int32_t *aLineNumber); \
  nsresult GetColumnNumberXPCOM(JSContext* cx, int32_t *aColumnNumber); \
  nsresult GetSourceLine(nsACString& aSourceLine); \
  nsresult GetAsyncCauseXPCOM(JSContext* cx, nsAString& aAsyncCause); \
  nsresult GetAsyncCallerXPCOM(JSContext* cx, nsIStackFrame **aAsyncCaller); \
  nsresult GetCallerXPCOM(JSContext* cx, nsIStackFrame **aCaller); \
  nsresult GetFormattedStackXPCOM(JSContext* cx, nsAString& aFormattedStack); \
  nsresult GetNativeSavedFrame(JS::MutableHandleValue aNativeSavedFrame); \
  nsresult ToStringXPCOM(JSContext* cx, nsACString& _retval); \
  void GetFilename(JSContext * aCx, nsAString& aFilename); \
  void GetName(JSContext * aCx, nsAString& aName); \
  int32_t GetSourceId(JSContext * aCx); \
  int32_t GetLineNumber(JSContext * aCx); \
  int32_t GetColumnNumber(JSContext * aCx); \
  void GetAsyncCause(JSContext * aCx, nsAString& aAsyncCause); \
  already_AddRefed<nsIStackFrame> GetAsyncCaller(JSContext * aCx); \
  already_AddRefed<nsIStackFrame> GetCaller(JSContext * aCx); \
  void GetFormattedStack(JSContext * aCx, nsAString& aFormattedStack); \
  void ToString(JSContext * aCx, nsACString& aString); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTACKFRAME(_to) \
  NS_IMETHOD GetFilenameXPCOM(JSContext* cx, nsAString& aFilename) override { return _to GetFilenameXPCOM(cx, aFilename); } \
  NS_IMETHOD GetNameXPCOM(JSContext* cx, nsAString& aName) override { return _to GetNameXPCOM(cx, aName); } \
  NS_IMETHOD GetSourceIdXPCOM(JSContext* cx, int32_t *aSourceId) override { return _to GetSourceIdXPCOM(cx, aSourceId); } \
  NS_IMETHOD GetLineNumberXPCOM(JSContext* cx, int32_t *aLineNumber) override { return _to GetLineNumberXPCOM(cx, aLineNumber); } \
  NS_IMETHOD GetColumnNumberXPCOM(JSContext* cx, int32_t *aColumnNumber) override { return _to GetColumnNumberXPCOM(cx, aColumnNumber); } \
  NS_IMETHOD GetSourceLine(nsACString& aSourceLine) override { return _to GetSourceLine(aSourceLine); } \
  NS_IMETHOD GetAsyncCauseXPCOM(JSContext* cx, nsAString& aAsyncCause) override { return _to GetAsyncCauseXPCOM(cx, aAsyncCause); } \
  NS_IMETHOD GetAsyncCallerXPCOM(JSContext* cx, nsIStackFrame **aAsyncCaller) override { return _to GetAsyncCallerXPCOM(cx, aAsyncCaller); } \
  NS_IMETHOD GetCallerXPCOM(JSContext* cx, nsIStackFrame **aCaller) override { return _to GetCallerXPCOM(cx, aCaller); } \
  NS_IMETHOD GetFormattedStackXPCOM(JSContext* cx, nsAString& aFormattedStack) override { return _to GetFormattedStackXPCOM(cx, aFormattedStack); } \
  NS_IMETHOD GetNativeSavedFrame(JS::MutableHandleValue aNativeSavedFrame) override { return _to GetNativeSavedFrame(aNativeSavedFrame); } \
  NS_IMETHOD ToStringXPCOM(JSContext* cx, nsACString& _retval) override { return _to ToStringXPCOM(cx, _retval); } \
  virtual void GetFilename(JSContext * aCx, nsAString& aFilename) override { return _to GetFilename(aCx, aFilename); } \
  virtual void GetName(JSContext * aCx, nsAString& aName) override { return _to GetName(aCx, aName); } \
  virtual int32_t GetSourceId(JSContext * aCx) override { return _to GetSourceId(aCx); } \
  virtual int32_t GetLineNumber(JSContext * aCx) override { return _to GetLineNumber(aCx); } \
  virtual int32_t GetColumnNumber(JSContext * aCx) override { return _to GetColumnNumber(aCx); } \
  virtual void GetAsyncCause(JSContext * aCx, nsAString& aAsyncCause) override { return _to GetAsyncCause(aCx, aAsyncCause); } \
  virtual already_AddRefed<nsIStackFrame> GetAsyncCaller(JSContext * aCx) override { return _to GetAsyncCaller(aCx); } \
  virtual already_AddRefed<nsIStackFrame> GetCaller(JSContext * aCx) override { return _to GetCaller(aCx); } \
  virtual void GetFormattedStack(JSContext * aCx, nsAString& aFormattedStack) override { return _to GetFormattedStack(aCx, aFormattedStack); } \
  virtual void ToString(JSContext * aCx, nsACString& aString) override { return _to ToString(aCx, aString); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTACKFRAME(_to) \
  NS_IMETHOD GetFilenameXPCOM(JSContext* cx, nsAString& aFilename) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFilenameXPCOM(cx, aFilename); } \
  NS_IMETHOD GetNameXPCOM(JSContext* cx, nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNameXPCOM(cx, aName); } \
  NS_IMETHOD GetSourceIdXPCOM(JSContext* cx, int32_t *aSourceId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceIdXPCOM(cx, aSourceId); } \
  NS_IMETHOD GetLineNumberXPCOM(JSContext* cx, int32_t *aLineNumber) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLineNumberXPCOM(cx, aLineNumber); } \
  NS_IMETHOD GetColumnNumberXPCOM(JSContext* cx, int32_t *aColumnNumber) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColumnNumberXPCOM(cx, aColumnNumber); } \
  NS_IMETHOD GetSourceLine(nsACString& aSourceLine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceLine(aSourceLine); } \
  NS_IMETHOD GetAsyncCauseXPCOM(JSContext* cx, nsAString& aAsyncCause) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsyncCauseXPCOM(cx, aAsyncCause); } \
  NS_IMETHOD GetAsyncCallerXPCOM(JSContext* cx, nsIStackFrame **aAsyncCaller) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsyncCallerXPCOM(cx, aAsyncCaller); } \
  NS_IMETHOD GetCallerXPCOM(JSContext* cx, nsIStackFrame **aCaller) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCallerXPCOM(cx, aCaller); } \
  NS_IMETHOD GetFormattedStackXPCOM(JSContext* cx, nsAString& aFormattedStack) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFormattedStackXPCOM(cx, aFormattedStack); } \
  NS_IMETHOD GetNativeSavedFrame(JS::MutableHandleValue aNativeSavedFrame) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNativeSavedFrame(aNativeSavedFrame); } \
  NS_IMETHOD ToStringXPCOM(JSContext* cx, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ToStringXPCOM(cx, _retval); } \
  virtual void GetFilename(JSContext * aCx, nsAString& aFilename) override; \
  virtual void GetName(JSContext * aCx, nsAString& aName) override; \
  virtual int32_t GetSourceId(JSContext * aCx) override; \
  virtual int32_t GetLineNumber(JSContext * aCx) override; \
  virtual int32_t GetColumnNumber(JSContext * aCx) override; \
  virtual void GetAsyncCause(JSContext * aCx, nsAString& aAsyncCause) override; \
  virtual already_AddRefed<nsIStackFrame> GetAsyncCaller(JSContext * aCx) override; \
  virtual already_AddRefed<nsIStackFrame> GetCaller(JSContext * aCx) override; \
  virtual void GetFormattedStack(JSContext * aCx, nsAString& aFormattedStack) override; \
  virtual void ToString(JSContext * aCx, nsACString& aString) override; 


/* starting interface:    nsIException */
#define NS_IEXCEPTION_IID_STR "4371b5bf-6845-487f-8d9d-3f1e4a9badd2"

#define NS_IEXCEPTION_IID \
  {0x4371b5bf, 0x6845, 0x487f, \
    { 0x8d, 0x9d, 0x3f, 0x1e, 0x4a, 0x9b, 0xad, 0xd2 }}

class NS_NO_VTABLE nsIException : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEXCEPTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIException;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIException, NS_IEXCEPTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEXCEPTION \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEXCEPTION \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEXCEPTION(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEXCEPTION(_to) \
  /* no methods! */


#endif /* __gen_nsIException_h__ */
