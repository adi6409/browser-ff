/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/kvstore/nsIKeyValue.idl
 */

#ifndef __gen_nsIKeyValue_h__
#define __gen_nsIKeyValue_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIVariant_h__
#include "nsIVariant.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIKeyValueDatabaseCallback; /* forward declaration */

class nsIKeyValueEnumeratorCallback; /* forward declaration */

class nsIKeyValuePairCallback; /* forward declaration */

class nsIKeyValueVariantCallback; /* forward declaration */

class nsIKeyValueVoidCallback; /* forward declaration */

class nsIKeyValuePair; /* forward declaration */


/* starting interface:    nsIKeyValueService */
#define NS_IKEYVALUESERVICE_IID_STR "46c893dd-4c14-4de0-b33d-a1be18c6d062"

#define NS_IKEYVALUESERVICE_IID \
  {0x46c893dd, 0x4c14, 0x4de0, \
    { 0xb3, 0x3d, 0xa1, 0xbe, 0x18, 0xc6, 0xd0, 0x62 }}

class NS_NO_VTABLE nsIKeyValueService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IKEYVALUESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIKeyValueService;

  /* void getOrCreate (in nsIKeyValueDatabaseCallback callback, in AUTF8String path, in AUTF8String name); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOrCreate(nsIKeyValueDatabaseCallback *callback, const nsACString& path, const nsACString& name) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIKeyValueService, NS_IKEYVALUESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIKEYVALUESERVICE \
  NS_IMETHOD GetOrCreate(nsIKeyValueDatabaseCallback *callback, const nsACString& path, const nsACString& name) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIKEYVALUESERVICE \
  nsresult GetOrCreate(nsIKeyValueDatabaseCallback *callback, const nsACString& path, const nsACString& name); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIKEYVALUESERVICE(_to) \
  NS_IMETHOD GetOrCreate(nsIKeyValueDatabaseCallback *callback, const nsACString& path, const nsACString& name) override { return _to GetOrCreate(callback, path, name); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIKEYVALUESERVICE(_to) \
  NS_IMETHOD GetOrCreate(nsIKeyValueDatabaseCallback *callback, const nsACString& path, const nsACString& name) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOrCreate(callback, path, name); } 


/* starting interface:    nsIKeyValueDatabase */
#define NS_IKEYVALUEDATABASE_IID_STR "c449398e-174c-425b-8195-da6aa0ccd9a5"

#define NS_IKEYVALUEDATABASE_IID \
  {0xc449398e, 0x174c, 0x425b, \
    { 0x81, 0x95, 0xda, 0x6a, 0xa0, 0xcc, 0xd9, 0xa5 }}

class NS_NO_VTABLE nsIKeyValueDatabase : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IKEYVALUEDATABASE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIKeyValueDatabase;

  /* void put (in nsIKeyValueVoidCallback callback, in AUTF8String key, in nsIVariant value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Put(nsIKeyValueVoidCallback *callback, const nsACString& key, nsIVariant *value) = 0;

  /* void writeMany (in nsIKeyValueVoidCallback callback, in Array<nsIKeyValuePair> pairs); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WriteMany(nsIKeyValueVoidCallback *callback, const nsTArray<RefPtr<nsIKeyValuePair>>& pairs) = 0;

  /* void get (in nsIKeyValueVariantCallback callback, in AUTF8String key, [optional] in nsIVariant defaultValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Get(nsIKeyValueVariantCallback *callback, const nsACString& key, nsIVariant *defaultValue) = 0;

  /* void has (in nsIKeyValueVariantCallback callback, in AUTF8String key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Has(nsIKeyValueVariantCallback *callback, const nsACString& key) = 0;

  /* void delete (in nsIKeyValueVoidCallback callback, in AUTF8String key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Delete(nsIKeyValueVoidCallback *callback, const nsACString& key) = 0;

  /* void clear (in nsIKeyValueVoidCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Clear(nsIKeyValueVoidCallback *callback) = 0;

  /* void enumerate (in nsIKeyValueEnumeratorCallback callback, [optional] in AUTF8String fromKey, [optional] in AUTF8String toKey); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Enumerate(nsIKeyValueEnumeratorCallback *callback, const nsACString& fromKey, const nsACString& toKey) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIKeyValueDatabase, NS_IKEYVALUEDATABASE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIKEYVALUEDATABASE \
  NS_IMETHOD Put(nsIKeyValueVoidCallback *callback, const nsACString& key, nsIVariant *value) override; \
  NS_IMETHOD WriteMany(nsIKeyValueVoidCallback *callback, const nsTArray<RefPtr<nsIKeyValuePair>>& pairs) override; \
  NS_IMETHOD Get(nsIKeyValueVariantCallback *callback, const nsACString& key, nsIVariant *defaultValue) override; \
  NS_IMETHOD Has(nsIKeyValueVariantCallback *callback, const nsACString& key) override; \
  NS_IMETHOD Delete(nsIKeyValueVoidCallback *callback, const nsACString& key) override; \
  NS_IMETHOD Clear(nsIKeyValueVoidCallback *callback) override; \
  NS_IMETHOD Enumerate(nsIKeyValueEnumeratorCallback *callback, const nsACString& fromKey, const nsACString& toKey) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIKEYVALUEDATABASE \
  nsresult Put(nsIKeyValueVoidCallback *callback, const nsACString& key, nsIVariant *value); \
  nsresult WriteMany(nsIKeyValueVoidCallback *callback, const nsTArray<RefPtr<nsIKeyValuePair>>& pairs); \
  nsresult Get(nsIKeyValueVariantCallback *callback, const nsACString& key, nsIVariant *defaultValue); \
  nsresult Has(nsIKeyValueVariantCallback *callback, const nsACString& key); \
  nsresult Delete(nsIKeyValueVoidCallback *callback, const nsACString& key); \
  nsresult Clear(nsIKeyValueVoidCallback *callback); \
  nsresult Enumerate(nsIKeyValueEnumeratorCallback *callback, const nsACString& fromKey, const nsACString& toKey); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIKEYVALUEDATABASE(_to) \
  NS_IMETHOD Put(nsIKeyValueVoidCallback *callback, const nsACString& key, nsIVariant *value) override { return _to Put(callback, key, value); } \
  NS_IMETHOD WriteMany(nsIKeyValueVoidCallback *callback, const nsTArray<RefPtr<nsIKeyValuePair>>& pairs) override { return _to WriteMany(callback, pairs); } \
  NS_IMETHOD Get(nsIKeyValueVariantCallback *callback, const nsACString& key, nsIVariant *defaultValue) override { return _to Get(callback, key, defaultValue); } \
  NS_IMETHOD Has(nsIKeyValueVariantCallback *callback, const nsACString& key) override { return _to Has(callback, key); } \
  NS_IMETHOD Delete(nsIKeyValueVoidCallback *callback, const nsACString& key) override { return _to Delete(callback, key); } \
  NS_IMETHOD Clear(nsIKeyValueVoidCallback *callback) override { return _to Clear(callback); } \
  NS_IMETHOD Enumerate(nsIKeyValueEnumeratorCallback *callback, const nsACString& fromKey, const nsACString& toKey) override { return _to Enumerate(callback, fromKey, toKey); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIKEYVALUEDATABASE(_to) \
  NS_IMETHOD Put(nsIKeyValueVoidCallback *callback, const nsACString& key, nsIVariant *value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Put(callback, key, value); } \
  NS_IMETHOD WriteMany(nsIKeyValueVoidCallback *callback, const nsTArray<RefPtr<nsIKeyValuePair>>& pairs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteMany(callback, pairs); } \
  NS_IMETHOD Get(nsIKeyValueVariantCallback *callback, const nsACString& key, nsIVariant *defaultValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Get(callback, key, defaultValue); } \
  NS_IMETHOD Has(nsIKeyValueVariantCallback *callback, const nsACString& key) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Has(callback, key); } \
  NS_IMETHOD Delete(nsIKeyValueVoidCallback *callback, const nsACString& key) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Delete(callback, key); } \
  NS_IMETHOD Clear(nsIKeyValueVoidCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clear(callback); } \
  NS_IMETHOD Enumerate(nsIKeyValueEnumeratorCallback *callback, const nsACString& fromKey, const nsACString& toKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Enumerate(callback, fromKey, toKey); } 


/* starting interface:    nsIKeyValuePair */
#define NS_IKEYVALUEPAIR_IID_STR "bc37b06a-23b5-4b32-8281-4b8479601c7e"

#define NS_IKEYVALUEPAIR_IID \
  {0xbc37b06a, 0x23b5, 0x4b32, \
    { 0x82, 0x81, 0x4b, 0x84, 0x79, 0x60, 0x1c, 0x7e }}

class NS_NO_VTABLE nsIKeyValuePair : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IKEYVALUEPAIR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIKeyValuePair;

  /* readonly attribute AUTF8String key; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKey(nsACString& aKey) = 0;

  /* readonly attribute nsIVariant value; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetValue(nsIVariant **aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIKeyValuePair, NS_IKEYVALUEPAIR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIKEYVALUEPAIR \
  NS_IMETHOD GetKey(nsACString& aKey) override; \
  NS_IMETHOD GetValue(nsIVariant **aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIKEYVALUEPAIR \
  nsresult GetKey(nsACString& aKey); \
  nsresult GetValue(nsIVariant **aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIKEYVALUEPAIR(_to) \
  NS_IMETHOD GetKey(nsACString& aKey) override { return _to GetKey(aKey); } \
  NS_IMETHOD GetValue(nsIVariant **aValue) override { return _to GetValue(aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIKEYVALUEPAIR(_to) \
  NS_IMETHOD GetKey(nsACString& aKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKey(aKey); } \
  NS_IMETHOD GetValue(nsIVariant **aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } 


/* starting interface:    nsIKeyValueEnumerator */
#define NS_IKEYVALUEENUMERATOR_IID_STR "b9ba7116-b7ff-4717-9a28-a08e6879b199"

#define NS_IKEYVALUEENUMERATOR_IID \
  {0xb9ba7116, 0xb7ff, 0x4717, \
    { 0x9a, 0x28, 0xa0, 0x8e, 0x68, 0x79, 0xb1, 0x99 }}

class NS_NO_VTABLE nsIKeyValueEnumerator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IKEYVALUEENUMERATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIKeyValueEnumerator;

  /* bool hasMoreElements (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasMoreElements(bool *_retval) = 0;

  /* nsIKeyValuePair getNext (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNext(nsIKeyValuePair **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIKeyValueEnumerator, NS_IKEYVALUEENUMERATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIKEYVALUEENUMERATOR \
  NS_IMETHOD HasMoreElements(bool *_retval) override; \
  NS_IMETHOD GetNext(nsIKeyValuePair **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIKEYVALUEENUMERATOR \
  nsresult HasMoreElements(bool *_retval); \
  nsresult GetNext(nsIKeyValuePair **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIKEYVALUEENUMERATOR(_to) \
  NS_IMETHOD HasMoreElements(bool *_retval) override { return _to HasMoreElements(_retval); } \
  NS_IMETHOD GetNext(nsIKeyValuePair **_retval) override { return _to GetNext(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIKEYVALUEENUMERATOR(_to) \
  NS_IMETHOD HasMoreElements(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasMoreElements(_retval); } \
  NS_IMETHOD GetNext(nsIKeyValuePair **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNext(_retval); } 


/* starting interface:    nsIKeyValueDatabaseCallback */
#define NS_IKEYVALUEDATABASECALLBACK_IID_STR "2becc1f8-2d80-4b63-92a8-24ee8f79ee45"

#define NS_IKEYVALUEDATABASECALLBACK_IID \
  {0x2becc1f8, 0x2d80, 0x4b63, \
    { 0x92, 0xa8, 0x24, 0xee, 0x8f, 0x79, 0xee, 0x45 }}

class NS_NO_VTABLE nsIKeyValueDatabaseCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IKEYVALUEDATABASECALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIKeyValueDatabaseCallback;

  /* void resolve (in nsIKeyValueDatabase database); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Resolve(nsIKeyValueDatabase *database) = 0;

  /* void reject (in AUTF8String message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Reject(const nsACString& message) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIKeyValueDatabaseCallback, NS_IKEYVALUEDATABASECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIKEYVALUEDATABASECALLBACK \
  NS_IMETHOD Resolve(nsIKeyValueDatabase *database) override; \
  NS_IMETHOD Reject(const nsACString& message) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIKEYVALUEDATABASECALLBACK \
  nsresult Resolve(nsIKeyValueDatabase *database); \
  nsresult Reject(const nsACString& message); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIKEYVALUEDATABASECALLBACK(_to) \
  NS_IMETHOD Resolve(nsIKeyValueDatabase *database) override { return _to Resolve(database); } \
  NS_IMETHOD Reject(const nsACString& message) override { return _to Reject(message); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIKEYVALUEDATABASECALLBACK(_to) \
  NS_IMETHOD Resolve(nsIKeyValueDatabase *database) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Resolve(database); } \
  NS_IMETHOD Reject(const nsACString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reject(message); } 


/* starting interface:    nsIKeyValueEnumeratorCallback */
#define NS_IKEYVALUEENUMERATORCALLBACK_IID_STR "b7ea2183-880b-4424-ab24-5aa1555b775d"

#define NS_IKEYVALUEENUMERATORCALLBACK_IID \
  {0xb7ea2183, 0x880b, 0x4424, \
    { 0xab, 0x24, 0x5a, 0xa1, 0x55, 0x5b, 0x77, 0x5d }}

class NS_NO_VTABLE nsIKeyValueEnumeratorCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IKEYVALUEENUMERATORCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIKeyValueEnumeratorCallback;

  /* void resolve (in nsIKeyValueEnumerator enumerator); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Resolve(nsIKeyValueEnumerator *enumerator) = 0;

  /* void reject (in AUTF8String message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Reject(const nsACString& message) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIKeyValueEnumeratorCallback, NS_IKEYVALUEENUMERATORCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIKEYVALUEENUMERATORCALLBACK \
  NS_IMETHOD Resolve(nsIKeyValueEnumerator *enumerator) override; \
  NS_IMETHOD Reject(const nsACString& message) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIKEYVALUEENUMERATORCALLBACK \
  nsresult Resolve(nsIKeyValueEnumerator *enumerator); \
  nsresult Reject(const nsACString& message); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIKEYVALUEENUMERATORCALLBACK(_to) \
  NS_IMETHOD Resolve(nsIKeyValueEnumerator *enumerator) override { return _to Resolve(enumerator); } \
  NS_IMETHOD Reject(const nsACString& message) override { return _to Reject(message); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIKEYVALUEENUMERATORCALLBACK(_to) \
  NS_IMETHOD Resolve(nsIKeyValueEnumerator *enumerator) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Resolve(enumerator); } \
  NS_IMETHOD Reject(const nsACString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reject(message); } 


/* starting interface:    nsIKeyValuePairCallback */
#define NS_IKEYVALUEPAIRCALLBACK_IID_STR "50f65485-ec1e-4307-812b-b8a15e1f382e"

#define NS_IKEYVALUEPAIRCALLBACK_IID \
  {0x50f65485, 0xec1e, 0x4307, \
    { 0x81, 0x2b, 0xb8, 0xa1, 0x5e, 0x1f, 0x38, 0x2e }}

class NS_NO_VTABLE nsIKeyValuePairCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IKEYVALUEPAIRCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIKeyValuePairCallback;

  /* void resolve (in nsIKeyValuePair pair); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Resolve(nsIKeyValuePair *pair) = 0;

  /* void reject (in AUTF8String message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Reject(const nsACString& message) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIKeyValuePairCallback, NS_IKEYVALUEPAIRCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIKEYVALUEPAIRCALLBACK \
  NS_IMETHOD Resolve(nsIKeyValuePair *pair) override; \
  NS_IMETHOD Reject(const nsACString& message) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIKEYVALUEPAIRCALLBACK \
  nsresult Resolve(nsIKeyValuePair *pair); \
  nsresult Reject(const nsACString& message); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIKEYVALUEPAIRCALLBACK(_to) \
  NS_IMETHOD Resolve(nsIKeyValuePair *pair) override { return _to Resolve(pair); } \
  NS_IMETHOD Reject(const nsACString& message) override { return _to Reject(message); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIKEYVALUEPAIRCALLBACK(_to) \
  NS_IMETHOD Resolve(nsIKeyValuePair *pair) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Resolve(pair); } \
  NS_IMETHOD Reject(const nsACString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reject(message); } 


/* starting interface:    nsIKeyValueVariantCallback */
#define NS_IKEYVALUEVARIANTCALLBACK_IID_STR "174ebfa1-74ea-42a7-aa90-85bbaf1da4bf"

#define NS_IKEYVALUEVARIANTCALLBACK_IID \
  {0x174ebfa1, 0x74ea, 0x42a7, \
    { 0xaa, 0x90, 0x85, 0xbb, 0xaf, 0x1d, 0xa4, 0xbf }}

class NS_NO_VTABLE nsIKeyValueVariantCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IKEYVALUEVARIANTCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIKeyValueVariantCallback;

  /* void resolve (in nsIVariant result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Resolve(nsIVariant *result) = 0;

  /* void reject (in AUTF8String message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Reject(const nsACString& message) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIKeyValueVariantCallback, NS_IKEYVALUEVARIANTCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIKEYVALUEVARIANTCALLBACK \
  NS_IMETHOD Resolve(nsIVariant *result) override; \
  NS_IMETHOD Reject(const nsACString& message) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIKEYVALUEVARIANTCALLBACK \
  nsresult Resolve(nsIVariant *result); \
  nsresult Reject(const nsACString& message); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIKEYVALUEVARIANTCALLBACK(_to) \
  NS_IMETHOD Resolve(nsIVariant *result) override { return _to Resolve(result); } \
  NS_IMETHOD Reject(const nsACString& message) override { return _to Reject(message); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIKEYVALUEVARIANTCALLBACK(_to) \
  NS_IMETHOD Resolve(nsIVariant *result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Resolve(result); } \
  NS_IMETHOD Reject(const nsACString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reject(message); } 


/* starting interface:    nsIKeyValueVoidCallback */
#define NS_IKEYVALUEVOIDCALLBACK_IID_STR "0c17497a-ccf8-451a-838d-9dfa7f846379"

#define NS_IKEYVALUEVOIDCALLBACK_IID \
  {0x0c17497a, 0xccf8, 0x451a, \
    { 0x83, 0x8d, 0x9d, 0xfa, 0x7f, 0x84, 0x63, 0x79 }}

class NS_NO_VTABLE nsIKeyValueVoidCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IKEYVALUEVOIDCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIKeyValueVoidCallback;

  /* void resolve (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Resolve(void) = 0;

  /* void reject (in AUTF8String message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Reject(const nsACString& message) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIKeyValueVoidCallback, NS_IKEYVALUEVOIDCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIKEYVALUEVOIDCALLBACK \
  NS_IMETHOD Resolve(void) override; \
  NS_IMETHOD Reject(const nsACString& message) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIKEYVALUEVOIDCALLBACK \
  nsresult Resolve(void); \
  nsresult Reject(const nsACString& message); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIKEYVALUEVOIDCALLBACK(_to) \
  NS_IMETHOD Resolve(void) override { return _to Resolve(); } \
  NS_IMETHOD Reject(const nsACString& message) override { return _to Reject(message); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIKEYVALUEVOIDCALLBACK(_to) \
  NS_IMETHOD Resolve(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Resolve(); } \
  NS_IMETHOD Reject(const nsACString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reject(message); } 


#endif /* __gen_nsIKeyValue_h__ */
