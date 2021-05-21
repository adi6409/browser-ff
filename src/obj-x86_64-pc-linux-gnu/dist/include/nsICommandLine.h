/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/commandlines/nsICommandLine.idl
 */

#ifndef __gen_nsICommandLine_h__
#define __gen_nsICommandLine_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsICommandLine */
#define NS_ICOMMANDLINE_IID_STR "bc3173bd-aa46-46a0-9d25-d9867a9659b6"

#define NS_ICOMMANDLINE_IID \
  {0xbc3173bd, 0xaa46, 0x46a0, \
    { 0x9d, 0x25, 0xd9, 0x86, 0x7a, 0x96, 0x59, 0xb6 }}

class NS_NO_VTABLE nsICommandLine : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOMMANDLINE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICommandLine;

  /* readonly attribute long length; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLength(int32_t *aLength) = 0;

  /* AString getArgument (in long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetArgument(int32_t aIndex, nsAString& _retval) = 0;

  /* long findFlag (in AString aFlag, in boolean aCaseSensitive); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FindFlag(const nsAString& aFlag, bool aCaseSensitive, int32_t *_retval) = 0;

  /* void removeArguments (in long aStart, in long aEnd); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveArguments(int32_t aStart, int32_t aEnd) = 0;

  /* boolean handleFlag (in AString aFlag, in boolean aCaseSensitive); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleFlag(const nsAString& aFlag, bool aCaseSensitive, bool *_retval) = 0;

  /* AString handleFlagWithParam (in AString aFlag, in boolean aCaseSensitive); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleFlagWithParam(const nsAString& aFlag, bool aCaseSensitive, nsAString& _retval) = 0;

  /* readonly attribute unsigned long state; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetState(uint32_t *aState) = 0;

  enum {
    STATE_INITIAL_LAUNCH = 0U,
    STATE_REMOTE_AUTO = 1U,
    STATE_REMOTE_EXPLICIT = 2U
  };

  /* attribute boolean preventDefault; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPreventDefault(bool *aPreventDefault) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPreventDefault(bool aPreventDefault) = 0;

  /* readonly attribute nsIFile workingDirectory; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWorkingDirectory(nsIFile **aWorkingDirectory) = 0;

  /* nsIFile resolveFile (in AString aArgument); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResolveFile(const nsAString& aArgument, nsIFile **_retval) = 0;

  /* nsIURI resolveURI (in AString aArgument); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResolveURI(const nsAString& aArgument, nsIURI **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICommandLine, NS_ICOMMANDLINE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOMMANDLINE \
  NS_IMETHOD GetLength(int32_t *aLength) override; \
  NS_IMETHOD GetArgument(int32_t aIndex, nsAString& _retval) override; \
  NS_IMETHOD FindFlag(const nsAString& aFlag, bool aCaseSensitive, int32_t *_retval) override; \
  NS_IMETHOD RemoveArguments(int32_t aStart, int32_t aEnd) override; \
  NS_IMETHOD HandleFlag(const nsAString& aFlag, bool aCaseSensitive, bool *_retval) override; \
  NS_IMETHOD HandleFlagWithParam(const nsAString& aFlag, bool aCaseSensitive, nsAString& _retval) override; \
  NS_IMETHOD GetState(uint32_t *aState) override; \
  NS_IMETHOD GetPreventDefault(bool *aPreventDefault) override; \
  NS_IMETHOD SetPreventDefault(bool aPreventDefault) override; \
  NS_IMETHOD GetWorkingDirectory(nsIFile **aWorkingDirectory) override; \
  NS_IMETHOD ResolveFile(const nsAString& aArgument, nsIFile **_retval) override; \
  NS_IMETHOD ResolveURI(const nsAString& aArgument, nsIURI **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOMMANDLINE \
  nsresult GetLength(int32_t *aLength); \
  nsresult GetArgument(int32_t aIndex, nsAString& _retval); \
  nsresult FindFlag(const nsAString& aFlag, bool aCaseSensitive, int32_t *_retval); \
  nsresult RemoveArguments(int32_t aStart, int32_t aEnd); \
  nsresult HandleFlag(const nsAString& aFlag, bool aCaseSensitive, bool *_retval); \
  nsresult HandleFlagWithParam(const nsAString& aFlag, bool aCaseSensitive, nsAString& _retval); \
  nsresult GetState(uint32_t *aState); \
  nsresult GetPreventDefault(bool *aPreventDefault); \
  nsresult SetPreventDefault(bool aPreventDefault); \
  nsresult GetWorkingDirectory(nsIFile **aWorkingDirectory); \
  nsresult ResolveFile(const nsAString& aArgument, nsIFile **_retval); \
  nsresult ResolveURI(const nsAString& aArgument, nsIURI **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOMMANDLINE(_to) \
  NS_IMETHOD GetLength(int32_t *aLength) override { return _to GetLength(aLength); } \
  NS_IMETHOD GetArgument(int32_t aIndex, nsAString& _retval) override { return _to GetArgument(aIndex, _retval); } \
  NS_IMETHOD FindFlag(const nsAString& aFlag, bool aCaseSensitive, int32_t *_retval) override { return _to FindFlag(aFlag, aCaseSensitive, _retval); } \
  NS_IMETHOD RemoveArguments(int32_t aStart, int32_t aEnd) override { return _to RemoveArguments(aStart, aEnd); } \
  NS_IMETHOD HandleFlag(const nsAString& aFlag, bool aCaseSensitive, bool *_retval) override { return _to HandleFlag(aFlag, aCaseSensitive, _retval); } \
  NS_IMETHOD HandleFlagWithParam(const nsAString& aFlag, bool aCaseSensitive, nsAString& _retval) override { return _to HandleFlagWithParam(aFlag, aCaseSensitive, _retval); } \
  NS_IMETHOD GetState(uint32_t *aState) override { return _to GetState(aState); } \
  NS_IMETHOD GetPreventDefault(bool *aPreventDefault) override { return _to GetPreventDefault(aPreventDefault); } \
  NS_IMETHOD SetPreventDefault(bool aPreventDefault) override { return _to SetPreventDefault(aPreventDefault); } \
  NS_IMETHOD GetWorkingDirectory(nsIFile **aWorkingDirectory) override { return _to GetWorkingDirectory(aWorkingDirectory); } \
  NS_IMETHOD ResolveFile(const nsAString& aArgument, nsIFile **_retval) override { return _to ResolveFile(aArgument, _retval); } \
  NS_IMETHOD ResolveURI(const nsAString& aArgument, nsIURI **_retval) override { return _to ResolveURI(aArgument, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOMMANDLINE(_to) \
  NS_IMETHOD GetLength(int32_t *aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLength(aLength); } \
  NS_IMETHOD GetArgument(int32_t aIndex, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetArgument(aIndex, _retval); } \
  NS_IMETHOD FindFlag(const nsAString& aFlag, bool aCaseSensitive, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FindFlag(aFlag, aCaseSensitive, _retval); } \
  NS_IMETHOD RemoveArguments(int32_t aStart, int32_t aEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveArguments(aStart, aEnd); } \
  NS_IMETHOD HandleFlag(const nsAString& aFlag, bool aCaseSensitive, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleFlag(aFlag, aCaseSensitive, _retval); } \
  NS_IMETHOD HandleFlagWithParam(const nsAString& aFlag, bool aCaseSensitive, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleFlagWithParam(aFlag, aCaseSensitive, _retval); } \
  NS_IMETHOD GetState(uint32_t *aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetState(aState); } \
  NS_IMETHOD GetPreventDefault(bool *aPreventDefault) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPreventDefault(aPreventDefault); } \
  NS_IMETHOD SetPreventDefault(bool aPreventDefault) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPreventDefault(aPreventDefault); } \
  NS_IMETHOD GetWorkingDirectory(nsIFile **aWorkingDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWorkingDirectory(aWorkingDirectory); } \
  NS_IMETHOD ResolveFile(const nsAString& aArgument, nsIFile **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResolveFile(aArgument, _retval); } \
  NS_IMETHOD ResolveURI(const nsAString& aArgument, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResolveURI(aArgument, _retval); } 


#endif /* __gen_nsICommandLine_h__ */
