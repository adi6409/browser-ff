/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIRaceCacheWithNetwork.idl
 */

#ifndef __gen_nsIRaceCacheWithNetwork_h__
#define __gen_nsIRaceCacheWithNetwork_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIRaceCacheWithNetwork */
#define NS_IRACECACHEWITHNETWORK_IID_STR "4d963475-8b16-4c58-b804-8a23d49436c5"

#define NS_IRACECACHEWITHNETWORK_IID \
  {0x4d963475, 0x8b16, 0x4c58, \
    { 0xb8, 0x04, 0x8a, 0x23, 0xd4, 0x94, 0x36, 0xc5 }}

class NS_NO_VTABLE nsIRaceCacheWithNetwork : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IRACECACHEWITHNETWORK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRaceCacheWithNetwork;

  /* void test_triggerNetwork (in long timeout); */
  NS_IMETHOD Test_triggerNetwork(int32_t timeout) = 0;

  /* void test_delayCacheEntryOpeningBy (in long timeout); */
  NS_IMETHOD Test_delayCacheEntryOpeningBy(int32_t timeout) = 0;

  /* void test_triggerDelayedOpenCacheEntry (); */
  NS_IMETHOD Test_triggerDelayedOpenCacheEntry(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRaceCacheWithNetwork, NS_IRACECACHEWITHNETWORK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIRACECACHEWITHNETWORK \
  NS_IMETHOD Test_triggerNetwork(int32_t timeout) override; \
  NS_IMETHOD Test_delayCacheEntryOpeningBy(int32_t timeout) override; \
  NS_IMETHOD Test_triggerDelayedOpenCacheEntry(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIRACECACHEWITHNETWORK \
  nsresult Test_triggerNetwork(int32_t timeout); \
  nsresult Test_delayCacheEntryOpeningBy(int32_t timeout); \
  nsresult Test_triggerDelayedOpenCacheEntry(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIRACECACHEWITHNETWORK(_to) \
  NS_IMETHOD Test_triggerNetwork(int32_t timeout) override { return _to Test_triggerNetwork(timeout); } \
  NS_IMETHOD Test_delayCacheEntryOpeningBy(int32_t timeout) override { return _to Test_delayCacheEntryOpeningBy(timeout); } \
  NS_IMETHOD Test_triggerDelayedOpenCacheEntry(void) override { return _to Test_triggerDelayedOpenCacheEntry(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIRACECACHEWITHNETWORK(_to) \
  NS_IMETHOD Test_triggerNetwork(int32_t timeout) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Test_triggerNetwork(timeout); } \
  NS_IMETHOD Test_delayCacheEntryOpeningBy(int32_t timeout) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Test_delayCacheEntryOpeningBy(timeout); } \
  NS_IMETHOD Test_triggerDelayedOpenCacheEntry(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Test_triggerDelayedOpenCacheEntry(); } 


#endif /* __gen_nsIRaceCacheWithNetwork_h__ */
