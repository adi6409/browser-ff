/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIProperties.idl
 */

#ifndef __gen_nsIProperties_h__
#define __gen_nsIProperties_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIProperties */
#define NS_IPROPERTIES_IID_STR "78650582-4e93-4b60-8e85-26ebd3eb14ca"

#define NS_IPROPERTIES_IID \
  {0x78650582, 0x4e93, 0x4b60, \
    { 0x8e, 0x85, 0x26, 0xeb, 0xd3, 0xeb, 0x14, 0xca }}

class NS_NO_VTABLE nsIProperties : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROPERTIES_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProperties;

  /* void get (in string prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Get(const char * prop, const nsIID & iid, void * * result) = 0;

  /* void set (in string prop, in nsISupports value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Set(const char * prop, nsISupports *value) = 0;

  /* boolean has (in string prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Has(const char * prop, bool *_retval) = 0;

  /* void undefine (in string prop); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Undefine(const char * prop) = 0;

  /* Array<ACString> getKeys (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKeys(nsTArray<nsCString >& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProperties, NS_IPROPERTIES_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROPERTIES \
  NS_IMETHOD Get(const char * prop, const nsIID & iid, void * * result) override; \
  NS_IMETHOD Set(const char * prop, nsISupports *value) override; \
  NS_IMETHOD Has(const char * prop, bool *_retval) override; \
  NS_IMETHOD Undefine(const char * prop) override; \
  NS_IMETHOD GetKeys(nsTArray<nsCString >& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROPERTIES \
  nsresult Get(const char * prop, const nsIID & iid, void * * result); \
  nsresult Set(const char * prop, nsISupports *value); \
  nsresult Has(const char * prop, bool *_retval); \
  nsresult Undefine(const char * prop); \
  nsresult GetKeys(nsTArray<nsCString >& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROPERTIES(_to) \
  NS_IMETHOD Get(const char * prop, const nsIID & iid, void * * result) override { return _to Get(prop, iid, result); } \
  NS_IMETHOD Set(const char * prop, nsISupports *value) override { return _to Set(prop, value); } \
  NS_IMETHOD Has(const char * prop, bool *_retval) override { return _to Has(prop, _retval); } \
  NS_IMETHOD Undefine(const char * prop) override { return _to Undefine(prop); } \
  NS_IMETHOD GetKeys(nsTArray<nsCString >& _retval) override { return _to GetKeys(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROPERTIES(_to) \
  NS_IMETHOD Get(const char * prop, const nsIID & iid, void * * result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Get(prop, iid, result); } \
  NS_IMETHOD Set(const char * prop, nsISupports *value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Set(prop, value); } \
  NS_IMETHOD Has(const char * prop, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Has(prop, _retval); } \
  NS_IMETHOD Undefine(const char * prop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Undefine(prop); } \
  NS_IMETHOD GetKeys(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeys(_retval); } 


#endif /* __gen_nsIProperties_h__ */
