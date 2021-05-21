/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageRow.idl
 */

#ifndef __gen_mozIStorageRow_h__
#define __gen_mozIStorageRow_h__


#ifndef __gen_mozIStorageValueArray_h__
#include "mozIStorageValueArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIVariant; /* forward declaration */


/* starting interface:    mozIStorageRow */
#define MOZISTORAGEROW_IID_STR "62d1b6bd-cbfe-4f9b-aee1-0ead4af4e6dc"

#define MOZISTORAGEROW_IID \
  {0x62d1b6bd, 0xcbfe, 0x4f9b, \
    { 0xae, 0xe1, 0x0e, 0xad, 0x4a, 0xf4, 0xe6, 0xdc }}

class NS_NO_VTABLE mozIStorageRow : public mozIStorageValueArray {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGEROW_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageRow;

  /* nsIVariant getResultByIndex (in unsigned long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetResultByIndex(uint32_t aIndex, nsIVariant **_retval) = 0;

  /* nsIVariant getResultByName (in AUTF8String aName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetResultByName(const nsACString& aName, nsIVariant **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageRow, MOZISTORAGEROW_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGEROW \
  NS_IMETHOD GetResultByIndex(uint32_t aIndex, nsIVariant **_retval) override; \
  NS_IMETHOD GetResultByName(const nsACString& aName, nsIVariant **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGEROW \
  nsresult GetResultByIndex(uint32_t aIndex, nsIVariant **_retval); \
  nsresult GetResultByName(const nsACString& aName, nsIVariant **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGEROW(_to) \
  NS_IMETHOD GetResultByIndex(uint32_t aIndex, nsIVariant **_retval) override { return _to GetResultByIndex(aIndex, _retval); } \
  NS_IMETHOD GetResultByName(const nsACString& aName, nsIVariant **_retval) override { return _to GetResultByName(aName, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGEROW(_to) \
  NS_IMETHOD GetResultByIndex(uint32_t aIndex, nsIVariant **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResultByIndex(aIndex, _retval); } \
  NS_IMETHOD GetResultByName(const nsACString& aName, nsIVariant **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResultByName(aName, _retval); } 


#endif /* __gen_mozIStorageRow_h__ */
