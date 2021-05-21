/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIWritablePropertyBag2.idl
 */

#ifndef __gen_nsIWritablePropertyBag2_h__
#define __gen_nsIWritablePropertyBag2_h__


#ifndef __gen_nsIPropertyBag2_h__
#include "nsIPropertyBag2.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIWritablePropertyBag2 */
#define NS_IWRITABLEPROPERTYBAG2_IID_STR "9cfd1587-360e-4957-a58f-4c2b1c5e7ed9"

#define NS_IWRITABLEPROPERTYBAG2_IID \
  {0x9cfd1587, 0x360e, 0x4957, \
    { 0xa5, 0x8f, 0x4c, 0x2b, 0x1c, 0x5e, 0x7e, 0xd9 }}

class NS_NO_VTABLE nsIWritablePropertyBag2 : public nsIPropertyBag2 {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWRITABLEPROPERTYBAG2_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWritablePropertyBag2;

  /* void setPropertyAsInt32 (in AString prop, in int32_t value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPropertyAsInt32(const nsAString& prop, int32_t value) = 0;

  /* void setPropertyAsUint32 (in AString prop, in uint32_t value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPropertyAsUint32(const nsAString& prop, uint32_t value) = 0;

  /* void setPropertyAsInt64 (in AString prop, in int64_t value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPropertyAsInt64(const nsAString& prop, int64_t value) = 0;

  /* void setPropertyAsUint64 (in AString prop, in uint64_t value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPropertyAsUint64(const nsAString& prop, uint64_t value) = 0;

  /* void setPropertyAsDouble (in AString prop, in double value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPropertyAsDouble(const nsAString& prop, double value) = 0;

  /* void setPropertyAsAString (in AString prop, in AString value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPropertyAsAString(const nsAString& prop, const nsAString& value) = 0;

  /* void setPropertyAsACString (in AString prop, in ACString value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPropertyAsACString(const nsAString& prop, const nsACString& value) = 0;

  /* void setPropertyAsAUTF8String (in AString prop, in AUTF8String value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPropertyAsAUTF8String(const nsAString& prop, const nsACString& value) = 0;

  /* void setPropertyAsBool (in AString prop, in boolean value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPropertyAsBool(const nsAString& prop, bool value) = 0;

  /* void setPropertyAsInterface (in AString prop, in nsISupports value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPropertyAsInterface(const nsAString& prop, nsISupports *value) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWritablePropertyBag2, NS_IWRITABLEPROPERTYBAG2_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWRITABLEPROPERTYBAG2 \
  NS_IMETHOD SetPropertyAsInt32(const nsAString& prop, int32_t value) override; \
  NS_IMETHOD SetPropertyAsUint32(const nsAString& prop, uint32_t value) override; \
  NS_IMETHOD SetPropertyAsInt64(const nsAString& prop, int64_t value) override; \
  NS_IMETHOD SetPropertyAsUint64(const nsAString& prop, uint64_t value) override; \
  NS_IMETHOD SetPropertyAsDouble(const nsAString& prop, double value) override; \
  NS_IMETHOD SetPropertyAsAString(const nsAString& prop, const nsAString& value) override; \
  NS_IMETHOD SetPropertyAsACString(const nsAString& prop, const nsACString& value) override; \
  NS_IMETHOD SetPropertyAsAUTF8String(const nsAString& prop, const nsACString& value) override; \
  NS_IMETHOD SetPropertyAsBool(const nsAString& prop, bool value) override; \
  NS_IMETHOD SetPropertyAsInterface(const nsAString& prop, nsISupports *value) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWRITABLEPROPERTYBAG2 \
  nsresult SetPropertyAsInt32(const nsAString& prop, int32_t value); \
  nsresult SetPropertyAsUint32(const nsAString& prop, uint32_t value); \
  nsresult SetPropertyAsInt64(const nsAString& prop, int64_t value); \
  nsresult SetPropertyAsUint64(const nsAString& prop, uint64_t value); \
  nsresult SetPropertyAsDouble(const nsAString& prop, double value); \
  nsresult SetPropertyAsAString(const nsAString& prop, const nsAString& value); \
  nsresult SetPropertyAsACString(const nsAString& prop, const nsACString& value); \
  nsresult SetPropertyAsAUTF8String(const nsAString& prop, const nsACString& value); \
  nsresult SetPropertyAsBool(const nsAString& prop, bool value); \
  nsresult SetPropertyAsInterface(const nsAString& prop, nsISupports *value); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWRITABLEPROPERTYBAG2(_to) \
  NS_IMETHOD SetPropertyAsInt32(const nsAString& prop, int32_t value) override { return _to SetPropertyAsInt32(prop, value); } \
  NS_IMETHOD SetPropertyAsUint32(const nsAString& prop, uint32_t value) override { return _to SetPropertyAsUint32(prop, value); } \
  NS_IMETHOD SetPropertyAsInt64(const nsAString& prop, int64_t value) override { return _to SetPropertyAsInt64(prop, value); } \
  NS_IMETHOD SetPropertyAsUint64(const nsAString& prop, uint64_t value) override { return _to SetPropertyAsUint64(prop, value); } \
  NS_IMETHOD SetPropertyAsDouble(const nsAString& prop, double value) override { return _to SetPropertyAsDouble(prop, value); } \
  NS_IMETHOD SetPropertyAsAString(const nsAString& prop, const nsAString& value) override { return _to SetPropertyAsAString(prop, value); } \
  NS_IMETHOD SetPropertyAsACString(const nsAString& prop, const nsACString& value) override { return _to SetPropertyAsACString(prop, value); } \
  NS_IMETHOD SetPropertyAsAUTF8String(const nsAString& prop, const nsACString& value) override { return _to SetPropertyAsAUTF8String(prop, value); } \
  NS_IMETHOD SetPropertyAsBool(const nsAString& prop, bool value) override { return _to SetPropertyAsBool(prop, value); } \
  NS_IMETHOD SetPropertyAsInterface(const nsAString& prop, nsISupports *value) override { return _to SetPropertyAsInterface(prop, value); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWRITABLEPROPERTYBAG2(_to) \
  NS_IMETHOD SetPropertyAsInt32(const nsAString& prop, int32_t value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPropertyAsInt32(prop, value); } \
  NS_IMETHOD SetPropertyAsUint32(const nsAString& prop, uint32_t value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPropertyAsUint32(prop, value); } \
  NS_IMETHOD SetPropertyAsInt64(const nsAString& prop, int64_t value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPropertyAsInt64(prop, value); } \
  NS_IMETHOD SetPropertyAsUint64(const nsAString& prop, uint64_t value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPropertyAsUint64(prop, value); } \
  NS_IMETHOD SetPropertyAsDouble(const nsAString& prop, double value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPropertyAsDouble(prop, value); } \
  NS_IMETHOD SetPropertyAsAString(const nsAString& prop, const nsAString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPropertyAsAString(prop, value); } \
  NS_IMETHOD SetPropertyAsACString(const nsAString& prop, const nsACString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPropertyAsACString(prop, value); } \
  NS_IMETHOD SetPropertyAsAUTF8String(const nsAString& prop, const nsACString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPropertyAsAUTF8String(prop, value); } \
  NS_IMETHOD SetPropertyAsBool(const nsAString& prop, bool value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPropertyAsBool(prop, value); } \
  NS_IMETHOD SetPropertyAsInterface(const nsAString& prop, nsISupports *value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPropertyAsInterface(prop, value); } 


#endif /* __gen_nsIWritablePropertyBag2_h__ */
