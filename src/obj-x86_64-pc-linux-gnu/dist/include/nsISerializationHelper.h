/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISerializationHelper.idl
 */

#ifndef __gen_nsISerializationHelper_h__
#define __gen_nsISerializationHelper_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsISerializable; /* forward declaration */


/* starting interface:    nsISerializationHelper */
#define NS_ISERIALIZATIONHELPER_IID_STR "31654c0f-35f3-44c6-b31e-37a11516e6bc"

#define NS_ISERIALIZATIONHELPER_IID \
  {0x31654c0f, 0x35f3, 0x44c6, \
    { 0xb3, 0x1e, 0x37, 0xa1, 0x15, 0x16, 0xe6, 0xbc }}

class NS_NO_VTABLE nsISerializationHelper : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISERIALIZATIONHELPER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISerializationHelper;

  /* ACString serializeToString (in nsISerializable serializable); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SerializeToString(nsISerializable *serializable, nsACString& _retval) = 0;

  /* nsISupports deserializeObject (in ACString input); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeserializeObject(const nsACString& input, nsISupports **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISerializationHelper, NS_ISERIALIZATIONHELPER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISERIALIZATIONHELPER \
  NS_IMETHOD SerializeToString(nsISerializable *serializable, nsACString& _retval) override; \
  NS_IMETHOD DeserializeObject(const nsACString& input, nsISupports **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISERIALIZATIONHELPER \
  nsresult SerializeToString(nsISerializable *serializable, nsACString& _retval); \
  nsresult DeserializeObject(const nsACString& input, nsISupports **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISERIALIZATIONHELPER(_to) \
  NS_IMETHOD SerializeToString(nsISerializable *serializable, nsACString& _retval) override { return _to SerializeToString(serializable, _retval); } \
  NS_IMETHOD DeserializeObject(const nsACString& input, nsISupports **_retval) override { return _to DeserializeObject(input, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISERIALIZATIONHELPER(_to) \
  NS_IMETHOD SerializeToString(nsISerializable *serializable, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SerializeToString(serializable, _retval); } \
  NS_IMETHOD DeserializeObject(const nsACString& input, nsISupports **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeserializeObject(input, _retval); } 


#endif /* __gen_nsISerializationHelper_h__ */
