/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIDialogParamBlock.idl
 */

#ifndef __gen_nsIDialogParamBlock_h__
#define __gen_nsIDialogParamBlock_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIMutableArray; /* forward declaration */


/* starting interface:    nsIDialogParamBlock */
#define NS_IDIALOGPARAMBLOCK_IID_STR "f76c0901-437a-11d3-b7a0-e35db351b4bc"

#define NS_IDIALOGPARAMBLOCK_IID \
  {0xf76c0901, 0x437a, 0x11d3, \
    { 0xb7, 0xa0, 0xe3, 0x5d, 0xb3, 0x51, 0xb4, 0xbc }}

class NS_NO_VTABLE nsIDialogParamBlock : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDIALOGPARAMBLOCK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDialogParamBlock;

  /* int32_t GetInt (in int32_t inIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInt(int32_t inIndex, int32_t *_retval) = 0;

  /* void SetInt (in int32_t inIndex, in int32_t inInt); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetInt(int32_t inIndex, int32_t inInt) = 0;

  /* void SetNumberStrings (in int32_t inNumStrings); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetNumberStrings(int32_t inNumStrings) = 0;

  /* wstring GetString (in int32_t inIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetString(int32_t inIndex, char16_t * *_retval) = 0;

  /* void SetString (in int32_t inIndex, in wstring inString); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetString(int32_t inIndex, const char16_t * inString) = 0;

  /* attribute nsIMutableArray objects; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetObjects(nsIMutableArray **aObjects) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetObjects(nsIMutableArray *aObjects) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDialogParamBlock, NS_IDIALOGPARAMBLOCK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDIALOGPARAMBLOCK \
  NS_IMETHOD GetInt(int32_t inIndex, int32_t *_retval) override; \
  NS_IMETHOD SetInt(int32_t inIndex, int32_t inInt) override; \
  NS_IMETHOD SetNumberStrings(int32_t inNumStrings) override; \
  NS_IMETHOD GetString(int32_t inIndex, char16_t * *_retval) override; \
  NS_IMETHOD SetString(int32_t inIndex, const char16_t * inString) override; \
  NS_IMETHOD GetObjects(nsIMutableArray **aObjects) override; \
  NS_IMETHOD SetObjects(nsIMutableArray *aObjects) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDIALOGPARAMBLOCK \
  nsresult GetInt(int32_t inIndex, int32_t *_retval); \
  nsresult SetInt(int32_t inIndex, int32_t inInt); \
  nsresult SetNumberStrings(int32_t inNumStrings); \
  nsresult GetString(int32_t inIndex, char16_t * *_retval); \
  nsresult SetString(int32_t inIndex, const char16_t * inString); \
  nsresult GetObjects(nsIMutableArray **aObjects); \
  nsresult SetObjects(nsIMutableArray *aObjects); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDIALOGPARAMBLOCK(_to) \
  NS_IMETHOD GetInt(int32_t inIndex, int32_t *_retval) override { return _to GetInt(inIndex, _retval); } \
  NS_IMETHOD SetInt(int32_t inIndex, int32_t inInt) override { return _to SetInt(inIndex, inInt); } \
  NS_IMETHOD SetNumberStrings(int32_t inNumStrings) override { return _to SetNumberStrings(inNumStrings); } \
  NS_IMETHOD GetString(int32_t inIndex, char16_t * *_retval) override { return _to GetString(inIndex, _retval); } \
  NS_IMETHOD SetString(int32_t inIndex, const char16_t * inString) override { return _to SetString(inIndex, inString); } \
  NS_IMETHOD GetObjects(nsIMutableArray **aObjects) override { return _to GetObjects(aObjects); } \
  NS_IMETHOD SetObjects(nsIMutableArray *aObjects) override { return _to SetObjects(aObjects); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDIALOGPARAMBLOCK(_to) \
  NS_IMETHOD GetInt(int32_t inIndex, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInt(inIndex, _retval); } \
  NS_IMETHOD SetInt(int32_t inIndex, int32_t inInt) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInt(inIndex, inInt); } \
  NS_IMETHOD SetNumberStrings(int32_t inNumStrings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNumberStrings(inNumStrings); } \
  NS_IMETHOD GetString(int32_t inIndex, char16_t * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetString(inIndex, _retval); } \
  NS_IMETHOD SetString(int32_t inIndex, const char16_t * inString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetString(inIndex, inString); } \
  NS_IMETHOD GetObjects(nsIMutableArray **aObjects) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetObjects(aObjects); } \
  NS_IMETHOD SetObjects(nsIMutableArray *aObjects) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetObjects(aObjects); } 

#define NS_DIALOGPARAMBLOCK_CONTRACTID "@mozilla.org/embedcomp/dialogparam;1"

#endif /* __gen_nsIDialogParamBlock_h__ */
