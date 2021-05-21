/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/http-sfv/nsIStructuredFieldValues.idl
 */

#ifndef __gen_nsIStructuredFieldValues_h__
#define __gen_nsIStructuredFieldValues_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsISFVBareItem */
#define NS_ISFVBAREITEM_IID_STR "7072853f-215b-4a8a-92e5-9732bccc377b"

#define NS_ISFVBAREITEM_IID \
  {0x7072853f, 0x215b, 0x4a8a, \
    { 0x92, 0xe5, 0x97, 0x32, 0xbc, 0xcc, 0x37, 0x7b }}

class NS_NO_VTABLE nsISFVBareItem : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVBAREITEM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVBareItem;

  enum {
    BOOL = 1,
    STRING = 2,
    DECIMAL = 3,
    INTEGER = 4,
    TOKEN = 5,
    BYTE_SEQUENCE = 6
  };

  /* readonly attribute long long type; */
  NS_IMETHOD GetType(int64_t *aType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVBareItem, NS_ISFVBAREITEM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVBAREITEM \
  NS_IMETHOD GetType(int64_t *aType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVBAREITEM \
  nsresult GetType(int64_t *aType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVBAREITEM(_to) \
  NS_IMETHOD GetType(int64_t *aType) override { return _to GetType(aType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVBAREITEM(_to) \
  NS_IMETHOD GetType(int64_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } 


/* starting interface:    nsISFVInteger */
#define NS_ISFVINTEGER_IID_STR "843eea44-990a-422c-bbf2-2aa4ba9ee4d2"

#define NS_ISFVINTEGER_IID \
  {0x843eea44, 0x990a, 0x422c, \
    { 0xbb, 0xf2, 0x2a, 0xa4, 0xba, 0x9e, 0xe4, 0xd2 }}

class NS_NO_VTABLE nsISFVInteger : public nsISFVBareItem {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVINTEGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVInteger;

  /* attribute long long value; */
  NS_IMETHOD GetValue(int64_t *aValue) = 0;
  NS_IMETHOD SetValue(int64_t aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVInteger, NS_ISFVINTEGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVINTEGER \
  NS_IMETHOD GetValue(int64_t *aValue) override; \
  NS_IMETHOD SetValue(int64_t aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVINTEGER \
  nsresult GetValue(int64_t *aValue); \
  nsresult SetValue(int64_t aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVINTEGER(_to) \
  NS_IMETHOD GetValue(int64_t *aValue) override { return _to GetValue(aValue); } \
  NS_IMETHOD SetValue(int64_t aValue) override { return _to SetValue(aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVINTEGER(_to) \
  NS_IMETHOD GetValue(int64_t *aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } \
  NS_IMETHOD SetValue(int64_t aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetValue(aValue); } 


/* starting interface:    nsISFVString */
#define NS_ISFVSTRING_IID_STR "df6a0787-7caa-4fef-b145-08c1104c2fde"

#define NS_ISFVSTRING_IID \
  {0xdf6a0787, 0x7caa, 0x4fef, \
    { 0xb1, 0x45, 0x08, 0xc1, 0x10, 0x4c, 0x2f, 0xde }}

class NS_NO_VTABLE nsISFVString : public nsISFVBareItem {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVSTRING_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVString;

  /* attribute ACString value; */
  NS_IMETHOD GetValue(nsACString& aValue) = 0;
  NS_IMETHOD SetValue(const nsACString& aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVString, NS_ISFVSTRING_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVSTRING \
  NS_IMETHOD GetValue(nsACString& aValue) override; \
  NS_IMETHOD SetValue(const nsACString& aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVSTRING \
  nsresult GetValue(nsACString& aValue); \
  nsresult SetValue(const nsACString& aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVSTRING(_to) \
  NS_IMETHOD GetValue(nsACString& aValue) override { return _to GetValue(aValue); } \
  NS_IMETHOD SetValue(const nsACString& aValue) override { return _to SetValue(aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVSTRING(_to) \
  NS_IMETHOD GetValue(nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } \
  NS_IMETHOD SetValue(const nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetValue(aValue); } 


/* starting interface:    nsISFVBool */
#define NS_ISFVBOOL_IID_STR "d263c6d7-4123-4c39-a121-ccf874a19012"

#define NS_ISFVBOOL_IID \
  {0xd263c6d7, 0x4123, 0x4c39, \
    { 0xa1, 0x21, 0xcc, 0xf8, 0x74, 0xa1, 0x90, 0x12 }}

class NS_NO_VTABLE nsISFVBool : public nsISFVBareItem {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVBOOL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVBool;

  /* attribute boolean value; */
  NS_IMETHOD GetValue(bool *aValue) = 0;
  NS_IMETHOD SetValue(bool aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVBool, NS_ISFVBOOL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVBOOL \
  NS_IMETHOD GetValue(bool *aValue) override; \
  NS_IMETHOD SetValue(bool aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVBOOL \
  nsresult GetValue(bool *aValue); \
  nsresult SetValue(bool aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVBOOL(_to) \
  NS_IMETHOD GetValue(bool *aValue) override { return _to GetValue(aValue); } \
  NS_IMETHOD SetValue(bool aValue) override { return _to SetValue(aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVBOOL(_to) \
  NS_IMETHOD GetValue(bool *aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } \
  NS_IMETHOD SetValue(bool aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetValue(aValue); } 


/* starting interface:    nsISFVDecimal */
#define NS_ISFVDECIMAL_IID_STR "1098da8b-b4df-4526-b985-53dbd4160ad2"

#define NS_ISFVDECIMAL_IID \
  {0x1098da8b, 0xb4df, 0x4526, \
    { 0xb9, 0x85, 0x53, 0xdb, 0xd4, 0x16, 0x0a, 0xd2 }}

class NS_NO_VTABLE nsISFVDecimal : public nsISFVBareItem {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVDECIMAL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVDecimal;

  /* attribute double value; */
  NS_IMETHOD GetValue(double *aValue) = 0;
  NS_IMETHOD SetValue(double aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVDecimal, NS_ISFVDECIMAL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVDECIMAL \
  NS_IMETHOD GetValue(double *aValue) override; \
  NS_IMETHOD SetValue(double aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVDECIMAL \
  nsresult GetValue(double *aValue); \
  nsresult SetValue(double aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVDECIMAL(_to) \
  NS_IMETHOD GetValue(double *aValue) override { return _to GetValue(aValue); } \
  NS_IMETHOD SetValue(double aValue) override { return _to SetValue(aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVDECIMAL(_to) \
  NS_IMETHOD GetValue(double *aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } \
  NS_IMETHOD SetValue(double aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetValue(aValue); } 


/* starting interface:    nsISFVToken */
#define NS_ISFVTOKEN_IID_STR "8ad33d52-b9b2-4a17-8aa8-991250fc1214"

#define NS_ISFVTOKEN_IID \
  {0x8ad33d52, 0xb9b2, 0x4a17, \
    { 0x8a, 0xa8, 0x99, 0x12, 0x50, 0xfc, 0x12, 0x14 }}

class NS_NO_VTABLE nsISFVToken : public nsISFVBareItem {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVTOKEN_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVToken;

  /* attribute ACString value; */
  NS_IMETHOD GetValue(nsACString& aValue) = 0;
  NS_IMETHOD SetValue(const nsACString& aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVToken, NS_ISFVTOKEN_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVTOKEN \
  NS_IMETHOD GetValue(nsACString& aValue) override; \
  NS_IMETHOD SetValue(const nsACString& aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVTOKEN \
  nsresult GetValue(nsACString& aValue); \
  nsresult SetValue(const nsACString& aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVTOKEN(_to) \
  NS_IMETHOD GetValue(nsACString& aValue) override { return _to GetValue(aValue); } \
  NS_IMETHOD SetValue(const nsACString& aValue) override { return _to SetValue(aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVTOKEN(_to) \
  NS_IMETHOD GetValue(nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } \
  NS_IMETHOD SetValue(const nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetValue(aValue); } 


/* starting interface:    nsISFVByteSeq */
#define NS_ISFVBYTESEQ_IID_STR "887eaef0-19fe-42bc-9a42-9ff773aa8fea"

#define NS_ISFVBYTESEQ_IID \
  {0x887eaef0, 0x19fe, 0x42bc, \
    { 0x9a, 0x42, 0x9f, 0xf7, 0x73, 0xaa, 0x8f, 0xea }}

class NS_NO_VTABLE nsISFVByteSeq : public nsISFVBareItem {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVBYTESEQ_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVByteSeq;

  /* attribute ACString value; */
  NS_IMETHOD GetValue(nsACString& aValue) = 0;
  NS_IMETHOD SetValue(const nsACString& aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVByteSeq, NS_ISFVBYTESEQ_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVBYTESEQ \
  NS_IMETHOD GetValue(nsACString& aValue) override; \
  NS_IMETHOD SetValue(const nsACString& aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVBYTESEQ \
  nsresult GetValue(nsACString& aValue); \
  nsresult SetValue(const nsACString& aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVBYTESEQ(_to) \
  NS_IMETHOD GetValue(nsACString& aValue) override { return _to GetValue(aValue); } \
  NS_IMETHOD SetValue(const nsACString& aValue) override { return _to SetValue(aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVBYTESEQ(_to) \
  NS_IMETHOD GetValue(nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } \
  NS_IMETHOD SetValue(const nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetValue(aValue); } 


/* starting interface:    nsISFVParams */
#define NS_ISFVPARAMS_IID_STR "b1a397d7-3333-43e7-993a-fbe8ab90ee96"

#define NS_ISFVPARAMS_IID \
  {0xb1a397d7, 0x3333, 0x43e7, \
    { 0x99, 0x3a, 0xfb, 0xe8, 0xab, 0x90, 0xee, 0x96 }}

class NS_NO_VTABLE nsISFVParams : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVPARAMS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVParams;

  /* nsISFVBareItem get (in ACString key); */
  NS_IMETHOD Get(const nsACString& key, nsISFVBareItem **_retval) = 0;

  /* void set (in ACString key, in nsISFVBareItem item); */
  NS_IMETHOD Set(const nsACString& key, nsISFVBareItem *item) = 0;

  /* void delete (in ACString key); */
  NS_IMETHOD Delete(const nsACString& key) = 0;

  /* Array<ACString> keys (); */
  NS_IMETHOD Keys(nsTArray<nsCString >& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVParams, NS_ISFVPARAMS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVPARAMS \
  NS_IMETHOD Get(const nsACString& key, nsISFVBareItem **_retval) override; \
  NS_IMETHOD Set(const nsACString& key, nsISFVBareItem *item) override; \
  NS_IMETHOD Delete(const nsACString& key) override; \
  NS_IMETHOD Keys(nsTArray<nsCString >& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVPARAMS \
  nsresult Get(const nsACString& key, nsISFVBareItem **_retval); \
  nsresult Set(const nsACString& key, nsISFVBareItem *item); \
  nsresult Delete(const nsACString& key); \
  nsresult Keys(nsTArray<nsCString >& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVPARAMS(_to) \
  NS_IMETHOD Get(const nsACString& key, nsISFVBareItem **_retval) override { return _to Get(key, _retval); } \
  NS_IMETHOD Set(const nsACString& key, nsISFVBareItem *item) override { return _to Set(key, item); } \
  NS_IMETHOD Delete(const nsACString& key) override { return _to Delete(key); } \
  NS_IMETHOD Keys(nsTArray<nsCString >& _retval) override { return _to Keys(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVPARAMS(_to) \
  NS_IMETHOD Get(const nsACString& key, nsISFVBareItem **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Get(key, _retval); } \
  NS_IMETHOD Set(const nsACString& key, nsISFVBareItem *item) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Set(key, item); } \
  NS_IMETHOD Delete(const nsACString& key) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Delete(key); } \
  NS_IMETHOD Keys(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Keys(_retval); } 


/* starting interface:    nsISFVParametrizable */
#define NS_ISFVPARAMETRIZABLE_IID_STR "6c0399f8-01de-4b25-b339-68e35e8d2e49"

#define NS_ISFVPARAMETRIZABLE_IID \
  {0x6c0399f8, 0x01de, 0x4b25, \
    { 0xb3, 0x39, 0x68, 0xe3, 0x5e, 0x8d, 0x2e, 0x49 }}

class NS_NO_VTABLE nsISFVParametrizable : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVPARAMETRIZABLE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVParametrizable;

  /* readonly attribute nsISFVParams params; */
  NS_IMETHOD GetParams(nsISFVParams **aParams) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVParametrizable, NS_ISFVPARAMETRIZABLE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVPARAMETRIZABLE \
  NS_IMETHOD GetParams(nsISFVParams **aParams) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVPARAMETRIZABLE \
  nsresult GetParams(nsISFVParams **aParams); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVPARAMETRIZABLE(_to) \
  NS_IMETHOD GetParams(nsISFVParams **aParams) override { return _to GetParams(aParams); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVPARAMETRIZABLE(_to) \
  NS_IMETHOD GetParams(nsISFVParams **aParams) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParams(aParams); } 


/* starting interface:    nsISFVItemOrInnerList */
#define NS_ISFVITEMORINNERLIST_IID_STR "99ac1b56-b5b3-44e7-ad96-db7444aae4b2"

#define NS_ISFVITEMORINNERLIST_IID \
  {0x99ac1b56, 0xb5b3, 0x44e7, \
    { 0xad, 0x96, 0xdb, 0x74, 0x44, 0xaa, 0xe4, 0xb2 }}

class NS_NO_VTABLE nsISFVItemOrInnerList : public nsISFVParametrizable {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVITEMORINNERLIST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVItemOrInnerList;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVItemOrInnerList, NS_ISFVITEMORINNERLIST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVITEMORINNERLIST \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVITEMORINNERLIST \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVITEMORINNERLIST(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVITEMORINNERLIST(_to) \
  /* no methods! */


/* starting interface:    nsISFVSerialize */
#define NS_ISFVSERIALIZE_IID_STR "28b9215d-c131-413c-9482-0004a371a5ec"

#define NS_ISFVSERIALIZE_IID \
  {0x28b9215d, 0xc131, 0x413c, \
    { 0x94, 0x82, 0x00, 0x04, 0xa3, 0x71, 0xa5, 0xec }}

class NS_NO_VTABLE nsISFVSerialize : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVSERIALIZE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVSerialize;

  /* ACString serialize (); */
  NS_IMETHOD Serialize(nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVSerialize, NS_ISFVSERIALIZE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVSERIALIZE \
  NS_IMETHOD Serialize(nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVSERIALIZE \
  nsresult Serialize(nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVSERIALIZE(_to) \
  NS_IMETHOD Serialize(nsACString& _retval) override { return _to Serialize(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVSERIALIZE(_to) \
  NS_IMETHOD Serialize(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Serialize(_retval); } 


/* starting interface:    nsISFVItem */
#define NS_ISFVITEM_IID_STR "abe8826b-6af7-4e54-bd2c-46ab231700ce"

#define NS_ISFVITEM_IID \
  {0xabe8826b, 0x6af7, 0x4e54, \
    { 0xbd, 0x2c, 0x46, 0xab, 0x23, 0x17, 0x00, 0xce }}

class NS_NO_VTABLE nsISFVItem : public nsISFVItemOrInnerList {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVITEM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVItem;

  /* readonly attribute nsISFVBareItem value; */
  NS_IMETHOD GetValue(nsISFVBareItem **aValue) = 0;

  /* ACString serialize (); */
  NS_IMETHOD Serialize(nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVItem, NS_ISFVITEM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVITEM \
  NS_IMETHOD GetValue(nsISFVBareItem **aValue) override; \
  NS_IMETHOD Serialize(nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVITEM \
  nsresult GetValue(nsISFVBareItem **aValue); \
  nsresult Serialize(nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVITEM(_to) \
  NS_IMETHOD GetValue(nsISFVBareItem **aValue) override { return _to GetValue(aValue); } \
  NS_IMETHOD Serialize(nsACString& _retval) override { return _to Serialize(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVITEM(_to) \
  NS_IMETHOD GetValue(nsISFVBareItem **aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } \
  NS_IMETHOD Serialize(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Serialize(_retval); } 


/* starting interface:    nsISFVInnerList */
#define NS_ISFVINNERLIST_IID_STR "b2e52be2-8488-41b2-9ee2-3c48d92d095c"

#define NS_ISFVINNERLIST_IID \
  {0xb2e52be2, 0x8488, 0x41b2, \
    { 0x9e, 0xe2, 0x3c, 0x48, 0xd9, 0x2d, 0x09, 0x5c }}

class NS_NO_VTABLE nsISFVInnerList : public nsISFVItemOrInnerList {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVINNERLIST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVInnerList;

  /* attribute Array<nsISFVItem> items; */
  NS_IMETHOD GetItems(nsTArray<RefPtr<nsISFVItem>>& aItems) = 0;
  NS_IMETHOD SetItems(const nsTArray<RefPtr<nsISFVItem>>& aItems) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVInnerList, NS_ISFVINNERLIST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVINNERLIST \
  NS_IMETHOD GetItems(nsTArray<RefPtr<nsISFVItem>>& aItems) override; \
  NS_IMETHOD SetItems(const nsTArray<RefPtr<nsISFVItem>>& aItems) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVINNERLIST \
  nsresult GetItems(nsTArray<RefPtr<nsISFVItem>>& aItems); \
  nsresult SetItems(const nsTArray<RefPtr<nsISFVItem>>& aItems); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVINNERLIST(_to) \
  NS_IMETHOD GetItems(nsTArray<RefPtr<nsISFVItem>>& aItems) override { return _to GetItems(aItems); } \
  NS_IMETHOD SetItems(const nsTArray<RefPtr<nsISFVItem>>& aItems) override { return _to SetItems(aItems); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVINNERLIST(_to) \
  NS_IMETHOD GetItems(nsTArray<RefPtr<nsISFVItem>>& aItems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetItems(aItems); } \
  NS_IMETHOD SetItems(const nsTArray<RefPtr<nsISFVItem>>& aItems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetItems(aItems); } 


/* starting interface:    nsISFVList */
#define NS_ISFVLIST_IID_STR "02bb92a6-d1de-449c-b54f-d137f30c613d"

#define NS_ISFVLIST_IID \
  {0x02bb92a6, 0xd1de, 0x449c, \
    { 0xb5, 0x4f, 0xd1, 0x37, 0xf3, 0x0c, 0x61, 0x3d }}

class NS_NO_VTABLE nsISFVList : public nsISFVSerialize {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVLIST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVList;

  /* attribute Array<nsISFVItemOrInnerList> members; */
  NS_IMETHOD GetMembers(nsTArray<RefPtr<nsISFVItemOrInnerList>>& aMembers) = 0;
  NS_IMETHOD SetMembers(const nsTArray<RefPtr<nsISFVItemOrInnerList>>& aMembers) = 0;

  /* void parseMore (in ACString header); */
  NS_IMETHOD ParseMore(const nsACString& header) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVList, NS_ISFVLIST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVLIST \
  NS_IMETHOD GetMembers(nsTArray<RefPtr<nsISFVItemOrInnerList>>& aMembers) override; \
  NS_IMETHOD SetMembers(const nsTArray<RefPtr<nsISFVItemOrInnerList>>& aMembers) override; \
  NS_IMETHOD ParseMore(const nsACString& header) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVLIST \
  nsresult GetMembers(nsTArray<RefPtr<nsISFVItemOrInnerList>>& aMembers); \
  nsresult SetMembers(const nsTArray<RefPtr<nsISFVItemOrInnerList>>& aMembers); \
  nsresult ParseMore(const nsACString& header); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVLIST(_to) \
  NS_IMETHOD GetMembers(nsTArray<RefPtr<nsISFVItemOrInnerList>>& aMembers) override { return _to GetMembers(aMembers); } \
  NS_IMETHOD SetMembers(const nsTArray<RefPtr<nsISFVItemOrInnerList>>& aMembers) override { return _to SetMembers(aMembers); } \
  NS_IMETHOD ParseMore(const nsACString& header) override { return _to ParseMore(header); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVLIST(_to) \
  NS_IMETHOD GetMembers(nsTArray<RefPtr<nsISFVItemOrInnerList>>& aMembers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMembers(aMembers); } \
  NS_IMETHOD SetMembers(const nsTArray<RefPtr<nsISFVItemOrInnerList>>& aMembers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMembers(aMembers); } \
  NS_IMETHOD ParseMore(const nsACString& header) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseMore(header); } 


/* starting interface:    nsISFVDictionary */
#define NS_ISFVDICTIONARY_IID_STR "6642a7fe-7026-4eba-b730-05e230ee3437"

#define NS_ISFVDICTIONARY_IID \
  {0x6642a7fe, 0x7026, 0x4eba, \
    { 0xb7, 0x30, 0x05, 0xe2, 0x30, 0xee, 0x34, 0x37 }}

class NS_NO_VTABLE nsISFVDictionary : public nsISFVSerialize {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVDICTIONARY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVDictionary;

  /* nsISFVItemOrInnerList get (in ACString key); */
  NS_IMETHOD Get(const nsACString& key, nsISFVItemOrInnerList **_retval) = 0;

  /* void set (in ACString key, in nsISFVItemOrInnerList member_value); */
  NS_IMETHOD Set(const nsACString& key, nsISFVItemOrInnerList *member_value) = 0;

  /* void delete (in ACString key); */
  NS_IMETHOD Delete(const nsACString& key) = 0;

  /* Array<ACString> keys (); */
  NS_IMETHOD Keys(nsTArray<nsCString >& _retval) = 0;

  /* void parseMore (in ACString header); */
  NS_IMETHOD ParseMore(const nsACString& header) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVDictionary, NS_ISFVDICTIONARY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVDICTIONARY \
  NS_IMETHOD Get(const nsACString& key, nsISFVItemOrInnerList **_retval) override; \
  NS_IMETHOD Set(const nsACString& key, nsISFVItemOrInnerList *member_value) override; \
  NS_IMETHOD Delete(const nsACString& key) override; \
  NS_IMETHOD Keys(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD ParseMore(const nsACString& header) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVDICTIONARY \
  nsresult Get(const nsACString& key, nsISFVItemOrInnerList **_retval); \
  nsresult Set(const nsACString& key, nsISFVItemOrInnerList *member_value); \
  nsresult Delete(const nsACString& key); \
  nsresult Keys(nsTArray<nsCString >& _retval); \
  nsresult ParseMore(const nsACString& header); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVDICTIONARY(_to) \
  NS_IMETHOD Get(const nsACString& key, nsISFVItemOrInnerList **_retval) override { return _to Get(key, _retval); } \
  NS_IMETHOD Set(const nsACString& key, nsISFVItemOrInnerList *member_value) override { return _to Set(key, member_value); } \
  NS_IMETHOD Delete(const nsACString& key) override { return _to Delete(key); } \
  NS_IMETHOD Keys(nsTArray<nsCString >& _retval) override { return _to Keys(_retval); } \
  NS_IMETHOD ParseMore(const nsACString& header) override { return _to ParseMore(header); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVDICTIONARY(_to) \
  NS_IMETHOD Get(const nsACString& key, nsISFVItemOrInnerList **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Get(key, _retval); } \
  NS_IMETHOD Set(const nsACString& key, nsISFVItemOrInnerList *member_value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Set(key, member_value); } \
  NS_IMETHOD Delete(const nsACString& key) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Delete(key); } \
  NS_IMETHOD Keys(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Keys(_retval); } \
  NS_IMETHOD ParseMore(const nsACString& header) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseMore(header); } 


/* starting interface:    nsISFVService */
#define NS_ISFVSERVICE_IID_STR "049f4be1-2f22-4438-a8da-518552ed390c"

#define NS_ISFVSERVICE_IID \
  {0x049f4be1, 0x2f22, 0x4438, \
    { 0xa8, 0xda, 0x51, 0x85, 0x52, 0xed, 0x39, 0x0c }}

class NS_NO_VTABLE nsISFVService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISFVSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISFVService;

  /* nsISFVDictionary parseDictionary (in ACString header); */
  NS_IMETHOD ParseDictionary(const nsACString& header, nsISFVDictionary **_retval) = 0;

  /* nsISFVList parseList (in ACString header); */
  NS_IMETHOD ParseList(const nsACString& header, nsISFVList **_retval) = 0;

  /* nsISFVItem parseItem (in ACString header); */
  NS_IMETHOD ParseItem(const nsACString& header, nsISFVItem **_retval) = 0;

  /* nsISFVInteger newInteger (in long long value); */
  NS_IMETHOD NewInteger(int64_t value, nsISFVInteger **_retval) = 0;

  /* nsISFVBool newBool (in bool value); */
  NS_IMETHOD NewBool(bool value, nsISFVBool **_retval) = 0;

  /* nsISFVDecimal newDecimal (in double value); */
  NS_IMETHOD NewDecimal(double value, nsISFVDecimal **_retval) = 0;

  /* nsISFVString newString (in ACString value); */
  NS_IMETHOD NewString(const nsACString& value, nsISFVString **_retval) = 0;

  /* nsISFVByteSeq newByteSequence (in ACString value); */
  NS_IMETHOD NewByteSequence(const nsACString& value, nsISFVByteSeq **_retval) = 0;

  /* nsISFVToken newToken (in ACString value); */
  NS_IMETHOD NewToken(const nsACString& value, nsISFVToken **_retval) = 0;

  /* nsISFVParams newParameters (); */
  NS_IMETHOD NewParameters(nsISFVParams **_retval) = 0;

  /* nsISFVInnerList newInnerList (in Array<nsISFVItem> items, in nsISFVParams params); */
  NS_IMETHOD NewInnerList(const nsTArray<RefPtr<nsISFVItem>>& items, nsISFVParams *params, nsISFVInnerList **_retval) = 0;

  /* nsISFVItem newItem (in nsISFVBareItem value, in nsISFVParams params); */
  NS_IMETHOD NewItem(nsISFVBareItem *value, nsISFVParams *params, nsISFVItem **_retval) = 0;

  /* nsISFVList newList (in Array<nsISFVItemOrInnerList> members); */
  NS_IMETHOD NewList(const nsTArray<RefPtr<nsISFVItemOrInnerList>>& members, nsISFVList **_retval) = 0;

  /* nsISFVDictionary newDictionary (); */
  NS_IMETHOD NewDictionary(nsISFVDictionary **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISFVService, NS_ISFVSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISFVSERVICE \
  NS_IMETHOD ParseDictionary(const nsACString& header, nsISFVDictionary **_retval) override; \
  NS_IMETHOD ParseList(const nsACString& header, nsISFVList **_retval) override; \
  NS_IMETHOD ParseItem(const nsACString& header, nsISFVItem **_retval) override; \
  NS_IMETHOD NewInteger(int64_t value, nsISFVInteger **_retval) override; \
  NS_IMETHOD NewBool(bool value, nsISFVBool **_retval) override; \
  NS_IMETHOD NewDecimal(double value, nsISFVDecimal **_retval) override; \
  NS_IMETHOD NewString(const nsACString& value, nsISFVString **_retval) override; \
  NS_IMETHOD NewByteSequence(const nsACString& value, nsISFVByteSeq **_retval) override; \
  NS_IMETHOD NewToken(const nsACString& value, nsISFVToken **_retval) override; \
  NS_IMETHOD NewParameters(nsISFVParams **_retval) override; \
  NS_IMETHOD NewInnerList(const nsTArray<RefPtr<nsISFVItem>>& items, nsISFVParams *params, nsISFVInnerList **_retval) override; \
  NS_IMETHOD NewItem(nsISFVBareItem *value, nsISFVParams *params, nsISFVItem **_retval) override; \
  NS_IMETHOD NewList(const nsTArray<RefPtr<nsISFVItemOrInnerList>>& members, nsISFVList **_retval) override; \
  NS_IMETHOD NewDictionary(nsISFVDictionary **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISFVSERVICE \
  nsresult ParseDictionary(const nsACString& header, nsISFVDictionary **_retval); \
  nsresult ParseList(const nsACString& header, nsISFVList **_retval); \
  nsresult ParseItem(const nsACString& header, nsISFVItem **_retval); \
  nsresult NewInteger(int64_t value, nsISFVInteger **_retval); \
  nsresult NewBool(bool value, nsISFVBool **_retval); \
  nsresult NewDecimal(double value, nsISFVDecimal **_retval); \
  nsresult NewString(const nsACString& value, nsISFVString **_retval); \
  nsresult NewByteSequence(const nsACString& value, nsISFVByteSeq **_retval); \
  nsresult NewToken(const nsACString& value, nsISFVToken **_retval); \
  nsresult NewParameters(nsISFVParams **_retval); \
  nsresult NewInnerList(const nsTArray<RefPtr<nsISFVItem>>& items, nsISFVParams *params, nsISFVInnerList **_retval); \
  nsresult NewItem(nsISFVBareItem *value, nsISFVParams *params, nsISFVItem **_retval); \
  nsresult NewList(const nsTArray<RefPtr<nsISFVItemOrInnerList>>& members, nsISFVList **_retval); \
  nsresult NewDictionary(nsISFVDictionary **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISFVSERVICE(_to) \
  NS_IMETHOD ParseDictionary(const nsACString& header, nsISFVDictionary **_retval) override { return _to ParseDictionary(header, _retval); } \
  NS_IMETHOD ParseList(const nsACString& header, nsISFVList **_retval) override { return _to ParseList(header, _retval); } \
  NS_IMETHOD ParseItem(const nsACString& header, nsISFVItem **_retval) override { return _to ParseItem(header, _retval); } \
  NS_IMETHOD NewInteger(int64_t value, nsISFVInteger **_retval) override { return _to NewInteger(value, _retval); } \
  NS_IMETHOD NewBool(bool value, nsISFVBool **_retval) override { return _to NewBool(value, _retval); } \
  NS_IMETHOD NewDecimal(double value, nsISFVDecimal **_retval) override { return _to NewDecimal(value, _retval); } \
  NS_IMETHOD NewString(const nsACString& value, nsISFVString **_retval) override { return _to NewString(value, _retval); } \
  NS_IMETHOD NewByteSequence(const nsACString& value, nsISFVByteSeq **_retval) override { return _to NewByteSequence(value, _retval); } \
  NS_IMETHOD NewToken(const nsACString& value, nsISFVToken **_retval) override { return _to NewToken(value, _retval); } \
  NS_IMETHOD NewParameters(nsISFVParams **_retval) override { return _to NewParameters(_retval); } \
  NS_IMETHOD NewInnerList(const nsTArray<RefPtr<nsISFVItem>>& items, nsISFVParams *params, nsISFVInnerList **_retval) override { return _to NewInnerList(items, params, _retval); } \
  NS_IMETHOD NewItem(nsISFVBareItem *value, nsISFVParams *params, nsISFVItem **_retval) override { return _to NewItem(value, params, _retval); } \
  NS_IMETHOD NewList(const nsTArray<RefPtr<nsISFVItemOrInnerList>>& members, nsISFVList **_retval) override { return _to NewList(members, _retval); } \
  NS_IMETHOD NewDictionary(nsISFVDictionary **_retval) override { return _to NewDictionary(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISFVSERVICE(_to) \
  NS_IMETHOD ParseDictionary(const nsACString& header, nsISFVDictionary **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseDictionary(header, _retval); } \
  NS_IMETHOD ParseList(const nsACString& header, nsISFVList **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseList(header, _retval); } \
  NS_IMETHOD ParseItem(const nsACString& header, nsISFVItem **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseItem(header, _retval); } \
  NS_IMETHOD NewInteger(int64_t value, nsISFVInteger **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewInteger(value, _retval); } \
  NS_IMETHOD NewBool(bool value, nsISFVBool **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewBool(value, _retval); } \
  NS_IMETHOD NewDecimal(double value, nsISFVDecimal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewDecimal(value, _retval); } \
  NS_IMETHOD NewString(const nsACString& value, nsISFVString **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewString(value, _retval); } \
  NS_IMETHOD NewByteSequence(const nsACString& value, nsISFVByteSeq **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewByteSequence(value, _retval); } \
  NS_IMETHOD NewToken(const nsACString& value, nsISFVToken **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewToken(value, _retval); } \
  NS_IMETHOD NewParameters(nsISFVParams **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewParameters(_retval); } \
  NS_IMETHOD NewInnerList(const nsTArray<RefPtr<nsISFVItem>>& items, nsISFVParams *params, nsISFVInnerList **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewInnerList(items, params, _retval); } \
  NS_IMETHOD NewItem(nsISFVBareItem *value, nsISFVParams *params, nsISFVItem **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewItem(value, params, _retval); } \
  NS_IMETHOD NewList(const nsTArray<RefPtr<nsISFVItemOrInnerList>>& members, nsISFVList **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewList(members, _retval); } \
  NS_IMETHOD NewDictionary(nsISFVDictionary **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewDictionary(_retval); } 


#endif /* __gen_nsIStructuredFieldValues_h__ */
