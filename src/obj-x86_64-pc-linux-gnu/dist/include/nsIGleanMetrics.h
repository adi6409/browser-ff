/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/glean/xpcom/nsIGleanMetrics.idl
 */

#ifndef __gen_nsIGleanMetrics_h__
#define __gen_nsIGleanMetrics_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIGleanBoolean */
#define NS_IGLEANBOOLEAN_IID_STR "d3180fe0-19fa-11eb-8b6f-0800200c9a66"

#define NS_IGLEANBOOLEAN_IID \
  {0xd3180fe0, 0x19fa, 0x11eb, \
    { 0x8b, 0x6f, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 }}

class NS_NO_VTABLE nsIGleanBoolean : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGLEANBOOLEAN_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGleanBoolean;

  /* void set (in bool value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Set(bool value) = 0;

  /* jsval testGetValue ([optional] in AUTF8String aPingName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGleanBoolean, NS_IGLEANBOOLEAN_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGLEANBOOLEAN \
  NS_IMETHOD Set(bool value) override; \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGLEANBOOLEAN \
  nsresult Set(bool value); \
  nsresult TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGLEANBOOLEAN(_to) \
  NS_IMETHOD Set(bool value) override { return _to Set(value); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval) override { return _to TestGetValue(aPingName, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGLEANBOOLEAN(_to) \
  NS_IMETHOD Set(bool value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Set(value); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestGetValue(aPingName, _retval); } 


/* starting interface:    nsIGleanDatetime */
#define NS_IGLEANDATETIME_IID_STR "aa15fd20-1e8a-11eb-9bec-0800200c9a66"

#define NS_IGLEANDATETIME_IID \
  {0xaa15fd20, 0x1e8a, 0x11eb, \
    { 0x9b, 0xec, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 }}

class NS_NO_VTABLE nsIGleanDatetime : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGLEANDATETIME_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGleanDatetime;

  /* [optional_argc] void set ([optional] in PRTime aValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Set(PRTime aValue, uint8_t _argc) = 0;

  /* [implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGleanDatetime, NS_IGLEANDATETIME_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGLEANDATETIME \
  NS_IMETHOD Set(PRTime aValue, uint8_t _argc) override; \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGLEANDATETIME \
  nsresult Set(PRTime aValue, uint8_t _argc); \
  nsresult TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGLEANDATETIME(_to) \
  NS_IMETHOD Set(PRTime aValue, uint8_t _argc) override { return _to Set(aValue, _argc); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return _to TestGetValue(aPingName, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGLEANDATETIME(_to) \
  NS_IMETHOD Set(PRTime aValue, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Set(aValue, _argc); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestGetValue(aPingName, cx, _retval); } 


/* starting interface:    nsIGleanCounter */
#define NS_IGLEANCOUNTER_IID_STR "05b89d2a-d57c-11ea-82da-3f63399a6f5a"

#define NS_IGLEANCOUNTER_IID \
  {0x05b89d2a, 0xd57c, 0x11ea, \
    { 0x82, 0xda, 0x3f, 0x63, 0x39, 0x9a, 0x6f, 0x5a }}

class NS_NO_VTABLE nsIGleanCounter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGLEANCOUNTER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGleanCounter;

  /* void add (in uint32_t amount); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Add(uint32_t amount) = 0;

  /* jsval testGetValue ([optional] in AUTF8String aPingName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGleanCounter, NS_IGLEANCOUNTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGLEANCOUNTER \
  NS_IMETHOD Add(uint32_t amount) override; \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGLEANCOUNTER \
  nsresult Add(uint32_t amount); \
  nsresult TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGLEANCOUNTER(_to) \
  NS_IMETHOD Add(uint32_t amount) override { return _to Add(amount); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval) override { return _to TestGetValue(aPingName, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGLEANCOUNTER(_to) \
  NS_IMETHOD Add(uint32_t amount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Add(amount); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestGetValue(aPingName, _retval); } 


/* starting interface:    nsIGleanTimingDistribution */
#define NS_IGLEANTIMINGDISTRIBUTION_IID_STR "92e14730-9b5f-45a1-b018-f588d0b964d8"

#define NS_IGLEANTIMINGDISTRIBUTION_IID \
  {0x92e14730, 0x9b5f, 0x45a1, \
    { 0xb0, 0x18, 0xf5, 0x88, 0xd0, 0xb9, 0x64, 0xd8 }}

class NS_NO_VTABLE nsIGleanTimingDistribution : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGLEANTIMINGDISTRIBUTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGleanTimingDistribution;

  /* [implicit_jscontext] jsval start (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Start(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* void stopAndAccumulate (in uint64_t aId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StopAndAccumulate(uint64_t aId) = 0;

  /* void cancel (in uint64_t aId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(uint64_t aId) = 0;

  /* [implicit_jscontext] jsval testGetValue ([optional] in ACString aPingName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGleanTimingDistribution, NS_IGLEANTIMINGDISTRIBUTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGLEANTIMINGDISTRIBUTION \
  NS_IMETHOD Start(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD StopAndAccumulate(uint64_t aId) override; \
  NS_IMETHOD Cancel(uint64_t aId) override; \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGLEANTIMINGDISTRIBUTION \
  nsresult Start(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult StopAndAccumulate(uint64_t aId); \
  nsresult Cancel(uint64_t aId); \
  nsresult TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGLEANTIMINGDISTRIBUTION(_to) \
  NS_IMETHOD Start(JSContext* cx, JS::MutableHandleValue _retval) override { return _to Start(cx, _retval); } \
  NS_IMETHOD StopAndAccumulate(uint64_t aId) override { return _to StopAndAccumulate(aId); } \
  NS_IMETHOD Cancel(uint64_t aId) override { return _to Cancel(aId); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return _to TestGetValue(aPingName, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGLEANTIMINGDISTRIBUTION(_to) \
  NS_IMETHOD Start(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Start(cx, _retval); } \
  NS_IMETHOD StopAndAccumulate(uint64_t aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopAndAccumulate(aId); } \
  NS_IMETHOD Cancel(uint64_t aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(aId); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestGetValue(aPingName, cx, _retval); } 


/* starting interface:    nsIGleanMemoryDistribution */
#define NS_IGLEANMEMORYDISTRIBUTION_IID_STR "eea5ed46-16ba-46cd-bb1f-504581987fe1"

#define NS_IGLEANMEMORYDISTRIBUTION_IID \
  {0xeea5ed46, 0x16ba, 0x46cd, \
    { 0xbb, 0x1f, 0x50, 0x45, 0x81, 0x98, 0x7f, 0xe1 }}

class NS_NO_VTABLE nsIGleanMemoryDistribution : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGLEANMEMORYDISTRIBUTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGleanMemoryDistribution;

  /* void accumulate (in uint64_t aSample); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Accumulate(uint64_t aSample) = 0;

  /* [implicit_jscontext] jsval testGetValue ([optional] in ACString aPingName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGleanMemoryDistribution, NS_IGLEANMEMORYDISTRIBUTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGLEANMEMORYDISTRIBUTION \
  NS_IMETHOD Accumulate(uint64_t aSample) override; \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGLEANMEMORYDISTRIBUTION \
  nsresult Accumulate(uint64_t aSample); \
  nsresult TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGLEANMEMORYDISTRIBUTION(_to) \
  NS_IMETHOD Accumulate(uint64_t aSample) override { return _to Accumulate(aSample); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return _to TestGetValue(aPingName, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGLEANMEMORYDISTRIBUTION(_to) \
  NS_IMETHOD Accumulate(uint64_t aSample) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Accumulate(aSample); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestGetValue(aPingName, cx, _retval); } 


/* starting interface:    nsIGleanPing */
#define NS_IGLEANPING_IID_STR "5223a48b-687d-47ff-a629-fd4a72d1ecfa"

#define NS_IGLEANPING_IID \
  {0x5223a48b, 0x687d, 0x47ff, \
    { 0xa6, 0x29, 0xfd, 0x4a, 0x72, 0xd1, 0xec, 0xfa }}

class NS_NO_VTABLE nsIGleanPing : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGLEANPING_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGleanPing;

  /* void submit ([optional] in ACString aReason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Submit(const nsACString& aReason) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGleanPing, NS_IGLEANPING_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGLEANPING \
  NS_IMETHOD Submit(const nsACString& aReason) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGLEANPING \
  nsresult Submit(const nsACString& aReason); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGLEANPING(_to) \
  NS_IMETHOD Submit(const nsACString& aReason) override { return _to Submit(aReason); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGLEANPING(_to) \
  NS_IMETHOD Submit(const nsACString& aReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Submit(aReason); } 


/* starting interface:    nsIGleanString */
#define NS_IGLEANSTRING_IID_STR "d84a3555-46f1-48c1-9122-e8e88b069d2b"

#define NS_IGLEANSTRING_IID \
  {0xd84a3555, 0x46f1, 0x48c1, \
    { 0x91, 0x22, 0xe8, 0xe8, 0x8b, 0x06, 0x9d, 0x2b }}

class NS_NO_VTABLE nsIGleanString : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGLEANSTRING_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGleanString;

  /* void set (in AUTF8String value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Set(const nsACString& value) = 0;

  /* [implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGleanString, NS_IGLEANSTRING_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGLEANSTRING \
  NS_IMETHOD Set(const nsACString& value) override; \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGLEANSTRING \
  nsresult Set(const nsACString& value); \
  nsresult TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGLEANSTRING(_to) \
  NS_IMETHOD Set(const nsACString& value) override { return _to Set(value); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return _to TestGetValue(aPingName, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGLEANSTRING(_to) \
  NS_IMETHOD Set(const nsACString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Set(value); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestGetValue(aPingName, cx, _retval); } 


/* starting interface:    nsIGleanStringList */
#define NS_IGLEANSTRINGLIST_IID_STR "46751205-2ac7-47dc-91d2-ef4a95ef2af9"

#define NS_IGLEANSTRINGLIST_IID \
  {0x46751205, 0x2ac7, 0x47dc, \
    { 0x91, 0xd2, 0xef, 0x4a, 0x95, 0xef, 0x2a, 0xf9 }}

class NS_NO_VTABLE nsIGleanStringList : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGLEANSTRINGLIST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGleanStringList;

  /* void add (in AUTF8String value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Add(const nsACString& value) = 0;

  /* void set (in Array<AUTF8String> value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Set(const nsTArray<nsCString >& value) = 0;

  /* [implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGleanStringList, NS_IGLEANSTRINGLIST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGLEANSTRINGLIST \
  NS_IMETHOD Add(const nsACString& value) override; \
  NS_IMETHOD Set(const nsTArray<nsCString >& value) override; \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGLEANSTRINGLIST \
  nsresult Add(const nsACString& value); \
  nsresult Set(const nsTArray<nsCString >& value); \
  nsresult TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGLEANSTRINGLIST(_to) \
  NS_IMETHOD Add(const nsACString& value) override { return _to Add(value); } \
  NS_IMETHOD Set(const nsTArray<nsCString >& value) override { return _to Set(value); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return _to TestGetValue(aPingName, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGLEANSTRINGLIST(_to) \
  NS_IMETHOD Add(const nsACString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Add(value); } \
  NS_IMETHOD Set(const nsTArray<nsCString >& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Set(value); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestGetValue(aPingName, cx, _retval); } 


/* starting interface:    nsIGleanTimespan */
#define NS_IGLEANTIMESPAN_IID_STR "2586530c-030f-11eb-93cb-cbf30d25225a"

#define NS_IGLEANTIMESPAN_IID \
  {0x2586530c, 0x030f, 0x11eb, \
    { 0x93, 0xcb, 0xcb, 0xf3, 0x0d, 0x25, 0x22, 0x5a }}

class NS_NO_VTABLE nsIGleanTimespan : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGLEANTIMESPAN_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGleanTimespan;

  /* void start (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Start(void) = 0;

  /* void stop (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Stop(void) = 0;

  /* jsval testGetValue ([optional] in AUTF8String aPingName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGleanTimespan, NS_IGLEANTIMESPAN_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGLEANTIMESPAN \
  NS_IMETHOD Start(void) override; \
  NS_IMETHOD Stop(void) override; \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGLEANTIMESPAN \
  nsresult Start(void); \
  nsresult Stop(void); \
  nsresult TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGLEANTIMESPAN(_to) \
  NS_IMETHOD Start(void) override { return _to Start(); } \
  NS_IMETHOD Stop(void) override { return _to Stop(); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval) override { return _to TestGetValue(aPingName, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGLEANTIMESPAN(_to) \
  NS_IMETHOD Start(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Start(); } \
  NS_IMETHOD Stop(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Stop(); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestGetValue(aPingName, _retval); } 


/* starting interface:    nsIGleanUuid */
#define NS_IGLEANUUID_IID_STR "395700e7-06f6-46be-adcc-ea58977fda6d"

#define NS_IGLEANUUID_IID \
  {0x395700e7, 0x06f6, 0x46be, \
    { 0xad, 0xcc, 0xea, 0x58, 0x97, 0x7f, 0xda, 0x6d }}

class NS_NO_VTABLE nsIGleanUuid : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGLEANUUID_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGleanUuid;

  /* void set (in AUTF8String aValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Set(const nsACString& aValue) = 0;

  /* void generateAndSet (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GenerateAndSet(void) = 0;

  /* [implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGleanUuid, NS_IGLEANUUID_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGLEANUUID \
  NS_IMETHOD Set(const nsACString& aValue) override; \
  NS_IMETHOD GenerateAndSet(void) override; \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGLEANUUID \
  nsresult Set(const nsACString& aValue); \
  nsresult GenerateAndSet(void); \
  nsresult TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGLEANUUID(_to) \
  NS_IMETHOD Set(const nsACString& aValue) override { return _to Set(aValue); } \
  NS_IMETHOD GenerateAndSet(void) override { return _to GenerateAndSet(); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return _to TestGetValue(aPingName, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGLEANUUID(_to) \
  NS_IMETHOD Set(const nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Set(aValue); } \
  NS_IMETHOD GenerateAndSet(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GenerateAndSet(); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestGetValue(aPingName, cx, _retval); } 


/* starting interface:    nsIGleanEvent */
#define NS_IGLEANEVENT_IID_STR "1b01424a-1f55-11eb-92a5-0754f6c3f240"

#define NS_IGLEANEVENT_IID \
  {0x1b01424a, 0x1f55, 0x11eb, \
    { 0x92, 0xa5, 0x07, 0x54, 0xf6, 0xc3, 0xf2, 0x40 }}

class NS_NO_VTABLE nsIGleanEvent : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGLEANEVENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGleanEvent;

  /* [implicit_jscontext] void record ([optional] in jsval aExtra); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Record(JS::HandleValue aExtra, JSContext* cx) = 0;

  /* [implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGleanEvent, NS_IGLEANEVENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGLEANEVENT \
  NS_IMETHOD Record(JS::HandleValue aExtra, JSContext* cx) override; \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGLEANEVENT \
  nsresult Record(JS::HandleValue aExtra, JSContext* cx); \
  nsresult TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGLEANEVENT(_to) \
  NS_IMETHOD Record(JS::HandleValue aExtra, JSContext* cx) override { return _to Record(aExtra, cx); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return _to TestGetValue(aPingName, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGLEANEVENT(_to) \
  NS_IMETHOD Record(JS::HandleValue aExtra, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Record(aExtra, cx); } \
  NS_IMETHOD TestGetValue(const nsACString& aPingName, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestGetValue(aPingName, cx, _retval); } 


#endif /* __gen_nsIGleanMetrics_h__ */
