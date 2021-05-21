/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/bindings/nsIScriptError.idl
 */

#ifndef __gen_nsIScriptError_h__
#define __gen_nsIScriptError_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIArray_h__
#include "nsIArray.h"
#endif

#ifndef __gen_nsIConsoleMessage_h__
#include "nsIConsoleMessage.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

#include "nsString.h" // for nsDependentCString

/* starting interface:    nsIScriptErrorNote */
#define NS_ISCRIPTERRORNOTE_IID_STR "e8933fc9-c302-4e12-a55b-4f88611d9c6c"

#define NS_ISCRIPTERRORNOTE_IID \
  {0xe8933fc9, 0xc302, 0x4e12, \
    { 0xa5, 0x5b, 0x4f, 0x88, 0x61, 0x1d, 0x9c, 0x6c }}

class NS_NO_VTABLE nsIScriptErrorNote : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCRIPTERRORNOTE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScriptErrorNote;

  /* readonly attribute AString errorMessage; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetErrorMessage(nsAString& aErrorMessage) = 0;

  /* readonly attribute AString sourceName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSourceName(nsAString& aSourceName) = 0;

  /* readonly attribute uint32_t sourceId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSourceId(uint32_t *aSourceId) = 0;

  /* readonly attribute uint32_t lineNumber; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLineNumber(uint32_t *aLineNumber) = 0;

  /* readonly attribute uint32_t columnNumber; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetColumnNumber(uint32_t *aColumnNumber) = 0;

  /* AUTF8String toString (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ToString(nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScriptErrorNote, NS_ISCRIPTERRORNOTE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCRIPTERRORNOTE \
  NS_IMETHOD GetErrorMessage(nsAString& aErrorMessage) override; \
  NS_IMETHOD GetSourceName(nsAString& aSourceName) override; \
  NS_IMETHOD GetSourceId(uint32_t *aSourceId) override; \
  NS_IMETHOD GetLineNumber(uint32_t *aLineNumber) override; \
  NS_IMETHOD GetColumnNumber(uint32_t *aColumnNumber) override; \
  NS_IMETHOD ToString(nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCRIPTERRORNOTE \
  nsresult GetErrorMessage(nsAString& aErrorMessage); \
  nsresult GetSourceName(nsAString& aSourceName); \
  nsresult GetSourceId(uint32_t *aSourceId); \
  nsresult GetLineNumber(uint32_t *aLineNumber); \
  nsresult GetColumnNumber(uint32_t *aColumnNumber); \
  nsresult ToString(nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCRIPTERRORNOTE(_to) \
  NS_IMETHOD GetErrorMessage(nsAString& aErrorMessage) override { return _to GetErrorMessage(aErrorMessage); } \
  NS_IMETHOD GetSourceName(nsAString& aSourceName) override { return _to GetSourceName(aSourceName); } \
  NS_IMETHOD GetSourceId(uint32_t *aSourceId) override { return _to GetSourceId(aSourceId); } \
  NS_IMETHOD GetLineNumber(uint32_t *aLineNumber) override { return _to GetLineNumber(aLineNumber); } \
  NS_IMETHOD GetColumnNumber(uint32_t *aColumnNumber) override { return _to GetColumnNumber(aColumnNumber); } \
  NS_IMETHOD ToString(nsACString& _retval) override { return _to ToString(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCRIPTERRORNOTE(_to) \
  NS_IMETHOD GetErrorMessage(nsAString& aErrorMessage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetErrorMessage(aErrorMessage); } \
  NS_IMETHOD GetSourceName(nsAString& aSourceName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceName(aSourceName); } \
  NS_IMETHOD GetSourceId(uint32_t *aSourceId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceId(aSourceId); } \
  NS_IMETHOD GetLineNumber(uint32_t *aLineNumber) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLineNumber(aLineNumber); } \
  NS_IMETHOD GetColumnNumber(uint32_t *aColumnNumber) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColumnNumber(aColumnNumber); } \
  NS_IMETHOD ToString(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ToString(_retval); } 


/* starting interface:    nsIScriptError */
#define NS_ISCRIPTERROR_IID_STR "63eb4d3e-7d99-4150-b4f3-11314f9d82a9"

#define NS_ISCRIPTERROR_IID \
  {0x63eb4d3e, 0x7d99, 0x4150, \
    { 0xb4, 0xf3, 0x11, 0x31, 0x4f, 0x9d, 0x82, 0xa9 }}

class nsIScriptError : public nsIConsoleMessage {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCRIPTERROR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScriptError;

  enum {
    errorFlag = 0U,
    warningFlag = 1U,
    infoFlag = 8U
  };

  /* readonly attribute AString errorMessage; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetErrorMessage(nsAString& aErrorMessage) = 0;

  /* readonly attribute AString sourceName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSourceName(nsAString& aSourceName) = 0;

  /* readonly attribute AString sourceLine; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSourceLine(nsAString& aSourceLine) = 0;

  /* readonly attribute uint32_t sourceId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSourceId(uint32_t *aSourceId) = 0;

  /* readonly attribute uint32_t lineNumber; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLineNumber(uint32_t *aLineNumber) = 0;

  /* readonly attribute uint32_t columnNumber; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetColumnNumber(uint32_t *aColumnNumber) = 0;

  /* readonly attribute uint32_t flags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFlags(uint32_t *aFlags) = 0;

  /* readonly attribute string category; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCategory(char * *aCategory) = 0;

  /* readonly attribute unsigned long long outerWindowID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOuterWindowID(uint64_t *aOuterWindowID) = 0;

  /* readonly attribute unsigned long long innerWindowID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInnerWindowID(uint64_t *aInnerWindowID) = 0;

  /* readonly attribute boolean isFromPrivateWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsFromPrivateWindow(bool *aIsFromPrivateWindow) = 0;

  /* readonly attribute boolean isFromChromeContext; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsFromChromeContext(bool *aIsFromChromeContext) = 0;

  /* attribute boolean isForwardedFromContentProcess; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsForwardedFromContentProcess(bool *aIsForwardedFromContentProcess) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetIsForwardedFromContentProcess(bool aIsForwardedFromContentProcess) = 0;

  /* readonly attribute boolean isPromiseRejection; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsPromiseRejection(bool *aIsPromiseRejection) = 0;

  /* [noscript] void initIsPromiseRejection (in bool isPromiseRejection); */
  NS_IMETHOD InitIsPromiseRejection(bool isPromiseRejection) = 0;

  /* attribute jsval exception; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetException(JS::MutableHandleValue aException) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetException(JS::HandleValue aException) = 0;

  /* readonly attribute boolean hasException; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHasException(bool *aHasException) = 0;

  /* attribute jsval stack; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStack(JS::MutableHandleValue aStack) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetStack(JS::HandleValue aStack) = 0;

  /* [noscript] readonly attribute jsval stackGlobal; */
  NS_IMETHOD GetStackGlobal(JS::MutableHandleValue aStackGlobal) = 0;

  /* attribute AString errorMessageName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetErrorMessageName(nsAString& aErrorMessageName) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetErrorMessageName(const nsAString& aErrorMessageName) = 0;

  /* readonly attribute nsIArray notes; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNotes(nsIArray **aNotes) = 0;

  /* attribute AString cssSelectors; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCssSelectors(nsAString& aCssSelectors) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCssSelectors(const nsAString& aCssSelectors) = 0;

  /* void init (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in string category, [optional] in bool fromPrivateWindow, [optional] in bool fromChromeContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const char * category, bool fromPrivateWindow, bool fromChromeContext) = 0;

  /* void initWithWindowID (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in ACString category, in unsigned long long innerWindowID, [optional] in bool fromChromeContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitWithWindowID(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext) = 0;

  /* void initWithSanitizedSource (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in ACString category, in unsigned long long innerWindowID, [optional] in bool fromChromeContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitWithSanitizedSource(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext) = 0;

  /* void initWithSourceURI (in AString message, in nsIURI sourceURI, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in ACString category, in unsigned long long innerWindowID, [optional] in bool fromChromeContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitWithSourceURI(const nsAString& message, nsIURI *sourceURI, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext) = 0;

  /* void initSourceId (in uint32_t sourceId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitSourceId(uint32_t sourceId) = 0;

     nsresult InitWithWindowID(const nsAString& message,
                              const nsAString& sourceName,
                              const nsAString& sourceLine,
                              uint32_t lineNumber,
                              uint32_t columnNumber,
                              uint32_t flags,
                              const nsACString& category,
                              uint64_t aInnerWindowID)
    {
        return InitWithWindowID(message, sourceName, sourceLine, lineNumber,
                                columnNumber, flags, category, aInnerWindowID,
                                false);
    }
    // These overloads allow passing a literal string for category.
    template<uint32_t N>
    nsresult InitWithWindowID(const nsAString& message,
                              const nsAString& sourceName,
                              const nsAString& sourceLine,
                              uint32_t lineNumber,
                              uint32_t columnNumber,
                              uint32_t flags,
                              const char (&c)[N],
                              uint64_t aInnerWindowID,
                              bool aFromChromeContext = false)
    {
        nsDependentCString category(c, N - 1);
        return InitWithWindowID(message, sourceName, sourceLine, lineNumber,
                                columnNumber, flags, category, aInnerWindowID,
                                aFromChromeContext);
    }
    nsresult InitWithSanitizedSource(const nsAString& message,
                                     const nsAString& sourceName,
                                     const nsAString& sourceLine,
                                     uint32_t lineNumber,
                                     uint32_t columnNumber,
                                     uint32_t flags,
                                     const nsACString& category,
                                     uint64_t aInnerWindowID)
    {
        return InitWithSanitizedSource(message, sourceName, sourceLine,
                                       lineNumber, columnNumber, flags,
                                       category, aInnerWindowID,
                                       false);
    }
    template<uint32_t N>
    nsresult InitWithSanitizedSource(const nsAString& message,
                                     const nsAString& sourceName,
                                     const nsAString& sourceLine,
                                     uint32_t lineNumber,
                                     uint32_t columnNumber,
                                     uint32_t flags,
                                     const char (&c)[N],
                                     uint64_t aInnerWindowID,
                                     bool aFromChromeContext = false)
    {
        nsDependentCString category(c, N - 1);
        return InitWithSanitizedSource(message, sourceName, sourceLine,
                                       lineNumber, columnNumber, flags,
                                       category, aInnerWindowID,
                                       aFromChromeContext);
    }
    nsresult InitWithSourceURI(const nsAString& message,
                               nsIURI* sourceURI,
                               const nsAString& sourceLine,
                               uint32_t lineNumber,
                               uint32_t columnNumber,
                               uint32_t flags,
                               const nsACString& category,
                               uint64_t aInnerWindowID)
    {
        return InitWithSourceURI(message, sourceURI, sourceLine,
                                 lineNumber, columnNumber, flags,
                                 category, aInnerWindowID, false);
    }
    template<uint32_t N>
    nsresult InitWithSourceURI(const nsAString& message,
                               nsIURI* sourceURI,
                               const nsAString& sourceLine,
                               uint32_t lineNumber,
                               uint32_t columnNumber,
                               uint32_t flags,
                               const char (&c)[N],
                               uint64_t aInnerWindowID,
                               bool aFromChromeContext = false)
    {
        nsDependentCString category(c, N - 1);
        return InitWithSourceURI(message, sourceURI, sourceLine,
                                 lineNumber, columnNumber, flags,
                                 category, aInnerWindowID, aFromChromeContext);
    }
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScriptError, NS_ISCRIPTERROR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCRIPTERROR \
  NS_IMETHOD GetErrorMessage(nsAString& aErrorMessage) override; \
  NS_IMETHOD GetSourceName(nsAString& aSourceName) override; \
  NS_IMETHOD GetSourceLine(nsAString& aSourceLine) override; \
  NS_IMETHOD GetSourceId(uint32_t *aSourceId) override; \
  NS_IMETHOD GetLineNumber(uint32_t *aLineNumber) override; \
  NS_IMETHOD GetColumnNumber(uint32_t *aColumnNumber) override; \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override; \
  NS_IMETHOD GetCategory(char * *aCategory) override; \
  NS_IMETHOD GetOuterWindowID(uint64_t *aOuterWindowID) override; \
  NS_IMETHOD GetInnerWindowID(uint64_t *aInnerWindowID) override; \
  NS_IMETHOD GetIsFromPrivateWindow(bool *aIsFromPrivateWindow) override; \
  NS_IMETHOD GetIsFromChromeContext(bool *aIsFromChromeContext) override; \
  NS_IMETHOD GetIsForwardedFromContentProcess(bool *aIsForwardedFromContentProcess) override; \
  NS_IMETHOD SetIsForwardedFromContentProcess(bool aIsForwardedFromContentProcess) override; \
  NS_IMETHOD GetIsPromiseRejection(bool *aIsPromiseRejection) override; \
  NS_IMETHOD InitIsPromiseRejection(bool isPromiseRejection) override; \
  NS_IMETHOD GetException(JS::MutableHandleValue aException) override; \
  NS_IMETHOD SetException(JS::HandleValue aException) override; \
  NS_IMETHOD GetHasException(bool *aHasException) override; \
  NS_IMETHOD GetStack(JS::MutableHandleValue aStack) override; \
  NS_IMETHOD SetStack(JS::HandleValue aStack) override; \
  NS_IMETHOD GetStackGlobal(JS::MutableHandleValue aStackGlobal) override; \
  NS_IMETHOD GetErrorMessageName(nsAString& aErrorMessageName) override; \
  NS_IMETHOD SetErrorMessageName(const nsAString& aErrorMessageName) override; \
  NS_IMETHOD GetNotes(nsIArray **aNotes) override; \
  NS_IMETHOD GetCssSelectors(nsAString& aCssSelectors) override; \
  NS_IMETHOD SetCssSelectors(const nsAString& aCssSelectors) override; \
  NS_IMETHOD Init(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const char * category, bool fromPrivateWindow, bool fromChromeContext) override; \
  NS_IMETHOD InitWithWindowID(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext) override; \
  NS_IMETHOD InitWithSanitizedSource(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext) override; \
  NS_IMETHOD InitWithSourceURI(const nsAString& message, nsIURI *sourceURI, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext) override; \
  NS_IMETHOD InitSourceId(uint32_t sourceId) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCRIPTERROR \
  nsresult GetErrorMessage(nsAString& aErrorMessage); \
  nsresult GetSourceName(nsAString& aSourceName); \
  nsresult GetSourceLine(nsAString& aSourceLine); \
  nsresult GetSourceId(uint32_t *aSourceId); \
  nsresult GetLineNumber(uint32_t *aLineNumber); \
  nsresult GetColumnNumber(uint32_t *aColumnNumber); \
  nsresult GetFlags(uint32_t *aFlags); \
  nsresult GetCategory(char * *aCategory); \
  nsresult GetOuterWindowID(uint64_t *aOuterWindowID); \
  nsresult GetInnerWindowID(uint64_t *aInnerWindowID); \
  nsresult GetIsFromPrivateWindow(bool *aIsFromPrivateWindow); \
  nsresult GetIsFromChromeContext(bool *aIsFromChromeContext); \
  nsresult GetIsForwardedFromContentProcess(bool *aIsForwardedFromContentProcess); \
  nsresult SetIsForwardedFromContentProcess(bool aIsForwardedFromContentProcess); \
  nsresult GetIsPromiseRejection(bool *aIsPromiseRejection); \
  nsresult InitIsPromiseRejection(bool isPromiseRejection); \
  nsresult GetException(JS::MutableHandleValue aException); \
  nsresult SetException(JS::HandleValue aException); \
  nsresult GetHasException(bool *aHasException); \
  nsresult GetStack(JS::MutableHandleValue aStack); \
  nsresult SetStack(JS::HandleValue aStack); \
  nsresult GetStackGlobal(JS::MutableHandleValue aStackGlobal); \
  nsresult GetErrorMessageName(nsAString& aErrorMessageName); \
  nsresult SetErrorMessageName(const nsAString& aErrorMessageName); \
  nsresult GetNotes(nsIArray **aNotes); \
  nsresult GetCssSelectors(nsAString& aCssSelectors); \
  nsresult SetCssSelectors(const nsAString& aCssSelectors); \
  nsresult Init(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const char * category, bool fromPrivateWindow, bool fromChromeContext); \
  nsresult InitWithWindowID(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext); \
  nsresult InitWithSanitizedSource(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext); \
  nsresult InitWithSourceURI(const nsAString& message, nsIURI *sourceURI, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext); \
  nsresult InitSourceId(uint32_t sourceId); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCRIPTERROR(_to) \
  NS_IMETHOD GetErrorMessage(nsAString& aErrorMessage) override { return _to GetErrorMessage(aErrorMessage); } \
  NS_IMETHOD GetSourceName(nsAString& aSourceName) override { return _to GetSourceName(aSourceName); } \
  NS_IMETHOD GetSourceLine(nsAString& aSourceLine) override { return _to GetSourceLine(aSourceLine); } \
  NS_IMETHOD GetSourceId(uint32_t *aSourceId) override { return _to GetSourceId(aSourceId); } \
  NS_IMETHOD GetLineNumber(uint32_t *aLineNumber) override { return _to GetLineNumber(aLineNumber); } \
  NS_IMETHOD GetColumnNumber(uint32_t *aColumnNumber) override { return _to GetColumnNumber(aColumnNumber); } \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override { return _to GetFlags(aFlags); } \
  NS_IMETHOD GetCategory(char * *aCategory) override { return _to GetCategory(aCategory); } \
  NS_IMETHOD GetOuterWindowID(uint64_t *aOuterWindowID) override { return _to GetOuterWindowID(aOuterWindowID); } \
  NS_IMETHOD GetInnerWindowID(uint64_t *aInnerWindowID) override { return _to GetInnerWindowID(aInnerWindowID); } \
  NS_IMETHOD GetIsFromPrivateWindow(bool *aIsFromPrivateWindow) override { return _to GetIsFromPrivateWindow(aIsFromPrivateWindow); } \
  NS_IMETHOD GetIsFromChromeContext(bool *aIsFromChromeContext) override { return _to GetIsFromChromeContext(aIsFromChromeContext); } \
  NS_IMETHOD GetIsForwardedFromContentProcess(bool *aIsForwardedFromContentProcess) override { return _to GetIsForwardedFromContentProcess(aIsForwardedFromContentProcess); } \
  NS_IMETHOD SetIsForwardedFromContentProcess(bool aIsForwardedFromContentProcess) override { return _to SetIsForwardedFromContentProcess(aIsForwardedFromContentProcess); } \
  NS_IMETHOD GetIsPromiseRejection(bool *aIsPromiseRejection) override { return _to GetIsPromiseRejection(aIsPromiseRejection); } \
  NS_IMETHOD InitIsPromiseRejection(bool isPromiseRejection) override { return _to InitIsPromiseRejection(isPromiseRejection); } \
  NS_IMETHOD GetException(JS::MutableHandleValue aException) override { return _to GetException(aException); } \
  NS_IMETHOD SetException(JS::HandleValue aException) override { return _to SetException(aException); } \
  NS_IMETHOD GetHasException(bool *aHasException) override { return _to GetHasException(aHasException); } \
  NS_IMETHOD GetStack(JS::MutableHandleValue aStack) override { return _to GetStack(aStack); } \
  NS_IMETHOD SetStack(JS::HandleValue aStack) override { return _to SetStack(aStack); } \
  NS_IMETHOD GetStackGlobal(JS::MutableHandleValue aStackGlobal) override { return _to GetStackGlobal(aStackGlobal); } \
  NS_IMETHOD GetErrorMessageName(nsAString& aErrorMessageName) override { return _to GetErrorMessageName(aErrorMessageName); } \
  NS_IMETHOD SetErrorMessageName(const nsAString& aErrorMessageName) override { return _to SetErrorMessageName(aErrorMessageName); } \
  NS_IMETHOD GetNotes(nsIArray **aNotes) override { return _to GetNotes(aNotes); } \
  NS_IMETHOD GetCssSelectors(nsAString& aCssSelectors) override { return _to GetCssSelectors(aCssSelectors); } \
  NS_IMETHOD SetCssSelectors(const nsAString& aCssSelectors) override { return _to SetCssSelectors(aCssSelectors); } \
  NS_IMETHOD Init(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const char * category, bool fromPrivateWindow, bool fromChromeContext) override { return _to Init(message, sourceName, sourceLine, lineNumber, columnNumber, flags, category, fromPrivateWindow, fromChromeContext); } \
  NS_IMETHOD InitWithWindowID(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext) override { return _to InitWithWindowID(message, sourceName, sourceLine, lineNumber, columnNumber, flags, category, innerWindowID, fromChromeContext); } \
  NS_IMETHOD InitWithSanitizedSource(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext) override { return _to InitWithSanitizedSource(message, sourceName, sourceLine, lineNumber, columnNumber, flags, category, innerWindowID, fromChromeContext); } \
  NS_IMETHOD InitWithSourceURI(const nsAString& message, nsIURI *sourceURI, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext) override { return _to InitWithSourceURI(message, sourceURI, sourceLine, lineNumber, columnNumber, flags, category, innerWindowID, fromChromeContext); } \
  NS_IMETHOD InitSourceId(uint32_t sourceId) override { return _to InitSourceId(sourceId); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCRIPTERROR(_to) \
  NS_IMETHOD GetErrorMessage(nsAString& aErrorMessage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetErrorMessage(aErrorMessage); } \
  NS_IMETHOD GetSourceName(nsAString& aSourceName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceName(aSourceName); } \
  NS_IMETHOD GetSourceLine(nsAString& aSourceLine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceLine(aSourceLine); } \
  NS_IMETHOD GetSourceId(uint32_t *aSourceId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceId(aSourceId); } \
  NS_IMETHOD GetLineNumber(uint32_t *aLineNumber) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLineNumber(aLineNumber); } \
  NS_IMETHOD GetColumnNumber(uint32_t *aColumnNumber) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColumnNumber(aColumnNumber); } \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFlags(aFlags); } \
  NS_IMETHOD GetCategory(char * *aCategory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCategory(aCategory); } \
  NS_IMETHOD GetOuterWindowID(uint64_t *aOuterWindowID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOuterWindowID(aOuterWindowID); } \
  NS_IMETHOD GetInnerWindowID(uint64_t *aInnerWindowID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInnerWindowID(aInnerWindowID); } \
  NS_IMETHOD GetIsFromPrivateWindow(bool *aIsFromPrivateWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsFromPrivateWindow(aIsFromPrivateWindow); } \
  NS_IMETHOD GetIsFromChromeContext(bool *aIsFromChromeContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsFromChromeContext(aIsFromChromeContext); } \
  NS_IMETHOD GetIsForwardedFromContentProcess(bool *aIsForwardedFromContentProcess) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsForwardedFromContentProcess(aIsForwardedFromContentProcess); } \
  NS_IMETHOD SetIsForwardedFromContentProcess(bool aIsForwardedFromContentProcess) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsForwardedFromContentProcess(aIsForwardedFromContentProcess); } \
  NS_IMETHOD GetIsPromiseRejection(bool *aIsPromiseRejection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsPromiseRejection(aIsPromiseRejection); } \
  NS_IMETHOD InitIsPromiseRejection(bool isPromiseRejection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitIsPromiseRejection(isPromiseRejection); } \
  NS_IMETHOD GetException(JS::MutableHandleValue aException) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetException(aException); } \
  NS_IMETHOD SetException(JS::HandleValue aException) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetException(aException); } \
  NS_IMETHOD GetHasException(bool *aHasException) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasException(aHasException); } \
  NS_IMETHOD GetStack(JS::MutableHandleValue aStack) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStack(aStack); } \
  NS_IMETHOD SetStack(JS::HandleValue aStack) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetStack(aStack); } \
  NS_IMETHOD GetStackGlobal(JS::MutableHandleValue aStackGlobal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStackGlobal(aStackGlobal); } \
  NS_IMETHOD GetErrorMessageName(nsAString& aErrorMessageName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetErrorMessageName(aErrorMessageName); } \
  NS_IMETHOD SetErrorMessageName(const nsAString& aErrorMessageName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetErrorMessageName(aErrorMessageName); } \
  NS_IMETHOD GetNotes(nsIArray **aNotes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotes(aNotes); } \
  NS_IMETHOD GetCssSelectors(nsAString& aCssSelectors) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCssSelectors(aCssSelectors); } \
  NS_IMETHOD SetCssSelectors(const nsAString& aCssSelectors) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCssSelectors(aCssSelectors); } \
  NS_IMETHOD Init(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const char * category, bool fromPrivateWindow, bool fromChromeContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(message, sourceName, sourceLine, lineNumber, columnNumber, flags, category, fromPrivateWindow, fromChromeContext); } \
  NS_IMETHOD InitWithWindowID(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithWindowID(message, sourceName, sourceLine, lineNumber, columnNumber, flags, category, innerWindowID, fromChromeContext); } \
  NS_IMETHOD InitWithSanitizedSource(const nsAString& message, const nsAString& sourceName, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithSanitizedSource(message, sourceName, sourceLine, lineNumber, columnNumber, flags, category, innerWindowID, fromChromeContext); } \
  NS_IMETHOD InitWithSourceURI(const nsAString& message, nsIURI *sourceURI, const nsAString& sourceLine, uint32_t lineNumber, uint32_t columnNumber, uint32_t flags, const nsACString& category, uint64_t innerWindowID, bool fromChromeContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithSourceURI(message, sourceURI, sourceLine, lineNumber, columnNumber, flags, category, innerWindowID, fromChromeContext); } \
  NS_IMETHOD InitSourceId(uint32_t sourceId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitSourceId(sourceId); } \

#define NS_SCRIPTERROR_CID \
{ 0x1950539a, 0x90f0, 0x4d22, { 0xb5, 0xaf, 0x71, 0x32, 0x9c, 0x68, 0xfa, 0x35 }}
#define NS_SCRIPTERROR_CONTRACTID "@mozilla.org/scripterror;1"

#endif /* __gen_nsIScriptError_h__ */
