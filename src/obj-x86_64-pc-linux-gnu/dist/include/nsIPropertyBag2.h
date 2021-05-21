/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIPropertyBag2.idl
 */

#ifndef __gen_nsIPropertyBag2_h__
#define __gen_nsIPropertyBag2_h__


#ifndef __gen_nsIPropertyBag_h__
#include "nsIPropertyBag.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPropertyBag2 */
#define NS_IPROPERTYBAG2_IID_STR "625cfd1e-da1e-4417-9ee9-dbc8e0b3fd79"

#define NS_IPROPERTYBAG2_IID \
  {0x625cfd1e, 0xda1e, 0x4417, \
    { 0x9e, 0xe9, 0xdb, 0xc8, 0xe0, 0xb3, 0xfd, 0x79 }}

class NS_NO_VTABLE nsIPropertyBag2 : public nsIPropertyBag {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROPERTYBAG2_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPropertyBag2;

  /* int32_t getPropertyAsInt32 (in AString prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPropertyAsInt32(const nsAString& prop, int32_t *_retval) = 0;

  /* uint32_t getPropertyAsUint32 (in AString prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPropertyAsUint32(const nsAString& prop, uint32_t *_retval) = 0;

  /* int64_t getPropertyAsInt64 (in AString prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPropertyAsInt64(const nsAString& prop, int64_t *_retval) = 0;

  /* uint64_t getPropertyAsUint64 (in AString prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPropertyAsUint64(const nsAString& prop, uint64_t *_retval) = 0;

  /* double getPropertyAsDouble (in AString prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPropertyAsDouble(const nsAString& prop, double *_retval) = 0;

  /* AString getPropertyAsAString (in AString prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPropertyAsAString(const nsAString& prop, nsAString& _retval) = 0;

  /* ACString getPropertyAsACString (in AString prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPropertyAsACString(const nsAString& prop, nsACString& _retval) = 0;

  /* AUTF8String getPropertyAsAUTF8String (in AString prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPropertyAsAUTF8String(const nsAString& prop, nsACString& _retval) = 0;

  /* boolean getPropertyAsBool (in AString prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPropertyAsBool(const nsAString& prop, bool *_retval) = 0;

  /* void getPropertyAsInterface (in AString prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPropertyAsInterface(const nsAString& prop, const nsIID & iid, void * * result) = 0;

  /* nsIVariant get (in AString prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Get(const nsAString& prop, nsIVariant **_retval) = 0;

  /* boolean hasKey (in AString prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasKey(const nsAString& prop, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPropertyBag2, NS_IPROPERTYBAG2_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROPERTYBAG2 \
  NS_IMETHOD GetPropertyAsInt32(const nsAString& prop, int32_t *_retval) override; \
  NS_IMETHOD GetPropertyAsUint32(const nsAString& prop, uint32_t *_retval) override; \
  NS_IMETHOD GetPropertyAsInt64(const nsAString& prop, int64_t *_retval) override; \
  NS_IMETHOD GetPropertyAsUint64(const nsAString& prop, uint64_t *_retval) override; \
  NS_IMETHOD GetPropertyAsDouble(const nsAString& prop, double *_retval) override; \
  NS_IMETHOD GetPropertyAsAString(const nsAString& prop, nsAString& _retval) override; \
  NS_IMETHOD GetPropertyAsACString(const nsAString& prop, nsACString& _retval) override; \
  NS_IMETHOD GetPropertyAsAUTF8String(const nsAString& prop, nsACString& _retval) override; \
  NS_IMETHOD GetPropertyAsBool(const nsAString& prop, bool *_retval) override; \
  NS_IMETHOD GetPropertyAsInterface(const nsAString& prop, const nsIID & iid, void * * result) override; \
  NS_IMETHOD Get(const nsAString& prop, nsIVariant **_retval) override; \
  NS_IMETHOD HasKey(const nsAString& prop, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROPERTYBAG2 \
  nsresult GetPropertyAsInt32(const nsAString& prop, int32_t *_retval); \
  nsresult GetPropertyAsUint32(const nsAString& prop, uint32_t *_retval); \
  nsresult GetPropertyAsInt64(const nsAString& prop, int64_t *_retval); \
  nsresult GetPropertyAsUint64(const nsAString& prop, uint64_t *_retval); \
  nsresult GetPropertyAsDouble(const nsAString& prop, double *_retval); \
  nsresult GetPropertyAsAString(const nsAString& prop, nsAString& _retval); \
  nsresult GetPropertyAsACString(const nsAString& prop, nsACString& _retval); \
  nsresult GetPropertyAsAUTF8String(const nsAString& prop, nsACString& _retval); \
  nsresult GetPropertyAsBool(const nsAString& prop, bool *_retval); \
  nsresult GetPropertyAsInterface(const nsAString& prop, const nsIID & iid, void * * result); \
  nsresult Get(const nsAString& prop, nsIVariant **_retval); \
  nsresult HasKey(const nsAString& prop, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROPERTYBAG2(_to) \
  NS_IMETHOD GetPropertyAsInt32(const nsAString& prop, int32_t *_retval) override { return _to GetPropertyAsInt32(prop, _retval); } \
  NS_IMETHOD GetPropertyAsUint32(const nsAString& prop, uint32_t *_retval) override { return _to GetPropertyAsUint32(prop, _retval); } \
  NS_IMETHOD GetPropertyAsInt64(const nsAString& prop, int64_t *_retval) override { return _to GetPropertyAsInt64(prop, _retval); } \
  NS_IMETHOD GetPropertyAsUint64(const nsAString& prop, uint64_t *_retval) override { return _to GetPropertyAsUint64(prop, _retval); } \
  NS_IMETHOD GetPropertyAsDouble(const nsAString& prop, double *_retval) override { return _to GetPropertyAsDouble(prop, _retval); } \
  NS_IMETHOD GetPropertyAsAString(const nsAString& prop, nsAString& _retval) override { return _to GetPropertyAsAString(prop, _retval); } \
  NS_IMETHOD GetPropertyAsACString(const nsAString& prop, nsACString& _retval) override { return _to GetPropertyAsACString(prop, _retval); } \
  NS_IMETHOD GetPropertyAsAUTF8String(const nsAString& prop, nsACString& _retval) override { return _to GetPropertyAsAUTF8String(prop, _retval); } \
  NS_IMETHOD GetPropertyAsBool(const nsAString& prop, bool *_retval) override { return _to GetPropertyAsBool(prop, _retval); } \
  NS_IMETHOD GetPropertyAsInterface(const nsAString& prop, const nsIID & iid, void * * result) override { return _to GetPropertyAsInterface(prop, iid, result); } \
  NS_IMETHOD Get(const nsAString& prop, nsIVariant **_retval) override { return _to Get(prop, _retval); } \
  NS_IMETHOD HasKey(const nsAString& prop, bool *_retval) override { return _to HasKey(prop, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROPERTYBAG2(_to) \
  NS_IMETHOD GetPropertyAsInt32(const nsAString& prop, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPropertyAsInt32(prop, _retval); } \
  NS_IMETHOD GetPropertyAsUint32(const nsAString& prop, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPropertyAsUint32(prop, _retval); } \
  NS_IMETHOD GetPropertyAsInt64(const nsAString& prop, int64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPropertyAsInt64(prop, _retval); } \
  NS_IMETHOD GetPropertyAsUint64(const nsAString& prop, uint64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPropertyAsUint64(prop, _retval); } \
  NS_IMETHOD GetPropertyAsDouble(const nsAString& prop, double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPropertyAsDouble(prop, _retval); } \
  NS_IMETHOD GetPropertyAsAString(const nsAString& prop, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPropertyAsAString(prop, _retval); } \
  NS_IMETHOD GetPropertyAsACString(const nsAString& prop, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPropertyAsACString(prop, _retval); } \
  NS_IMETHOD GetPropertyAsAUTF8String(const nsAString& prop, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPropertyAsAUTF8String(prop, _retval); } \
  NS_IMETHOD GetPropertyAsBool(const nsAString& prop, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPropertyAsBool(prop, _retval); } \
  NS_IMETHOD GetPropertyAsInterface(const nsAString& prop, const nsIID & iid, void * * result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPropertyAsInterface(prop, iid, result); } \
  NS_IMETHOD Get(const nsAString& prop, nsIVariant **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Get(prop, _retval); } \
  NS_IMETHOD HasKey(const nsAString& prop, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasKey(prop, _retval); } 


#endif /* __gen_nsIPropertyBag2_h__ */
