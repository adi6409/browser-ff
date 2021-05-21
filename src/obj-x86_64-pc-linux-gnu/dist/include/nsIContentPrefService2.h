/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIContentPrefService2.idl
 */

#ifndef __gen_nsIContentPrefService2_h__
#define __gen_nsIContentPrefService2_h__


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
class nsIVariant; /* forward declaration */

class nsIContentPrefCallback2; /* forward declaration */

class nsILoadContext; /* forward declaration */

class nsIContentPref; /* forward declaration */


/* starting interface:    nsIContentPrefObserver */
#define NS_ICONTENTPREFOBSERVER_IID_STR "43635c53-b445-4c4e-8cc5-562697299b55"

#define NS_ICONTENTPREFOBSERVER_IID \
  {0x43635c53, 0xb445, 0x4c4e, \
    { 0x8c, 0xc5, 0x56, 0x26, 0x97, 0x29, 0x9b, 0x55 }}

class NS_NO_VTABLE nsIContentPrefObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTPREFOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentPrefObserver;

  /* void onContentPrefSet (in AString aGroup, in AString aName, in nsIVariant aValue, [optional] in boolean aIsPrivate); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnContentPrefSet(const nsAString& aGroup, const nsAString& aName, nsIVariant *aValue, bool aIsPrivate) = 0;

  /* void onContentPrefRemoved (in AString aGroup, in AString aName, [optional] in boolean aIsPrivate); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnContentPrefRemoved(const nsAString& aGroup, const nsAString& aName, bool aIsPrivate) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentPrefObserver, NS_ICONTENTPREFOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTPREFOBSERVER \
  NS_IMETHOD OnContentPrefSet(const nsAString& aGroup, const nsAString& aName, nsIVariant *aValue, bool aIsPrivate) override; \
  NS_IMETHOD OnContentPrefRemoved(const nsAString& aGroup, const nsAString& aName, bool aIsPrivate) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTPREFOBSERVER \
  nsresult OnContentPrefSet(const nsAString& aGroup, const nsAString& aName, nsIVariant *aValue, bool aIsPrivate); \
  nsresult OnContentPrefRemoved(const nsAString& aGroup, const nsAString& aName, bool aIsPrivate); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTPREFOBSERVER(_to) \
  NS_IMETHOD OnContentPrefSet(const nsAString& aGroup, const nsAString& aName, nsIVariant *aValue, bool aIsPrivate) override { return _to OnContentPrefSet(aGroup, aName, aValue, aIsPrivate); } \
  NS_IMETHOD OnContentPrefRemoved(const nsAString& aGroup, const nsAString& aName, bool aIsPrivate) override { return _to OnContentPrefRemoved(aGroup, aName, aIsPrivate); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTPREFOBSERVER(_to) \
  NS_IMETHOD OnContentPrefSet(const nsAString& aGroup, const nsAString& aName, nsIVariant *aValue, bool aIsPrivate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnContentPrefSet(aGroup, aName, aValue, aIsPrivate); } \
  NS_IMETHOD OnContentPrefRemoved(const nsAString& aGroup, const nsAString& aName, bool aIsPrivate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnContentPrefRemoved(aGroup, aName, aIsPrivate); } 


/* starting interface:    nsIContentPrefService2 */
#define NS_ICONTENTPREFSERVICE2_IID_STR "bed98666-d995-470f-bebd-62476d318576"

#define NS_ICONTENTPREFSERVICE2_IID \
  {0xbed98666, 0xd995, 0x470f, \
    { 0xbe, 0xbd, 0x62, 0x47, 0x6d, 0x31, 0x85, 0x76 }}

class NS_NO_VTABLE nsIContentPrefService2 : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTPREFSERVICE2_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentPrefService2;

  /* void getByName (in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetByName(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void getByDomainAndName (in AString domain, in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void getBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void getGlobal (in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetGlobal(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* nsIContentPref getCachedByDomainAndName (in AString domain, in AString name, in nsILoadContext context); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCachedByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPref **_retval) = 0;

  /* Array<nsIContentPref> getCachedBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCachedBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsTArray<RefPtr<nsIContentPref>>& _retval) = 0;

  /* nsIContentPref getCachedGlobal (in AString name, in nsILoadContext context); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCachedGlobal(const nsAString& name, nsILoadContext *context, nsIContentPref **_retval) = 0;

  /* void set (in AString domain, in AString name, in nsIVariant value, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Set(const nsAString& domain, const nsAString& name, nsIVariant *value, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void setGlobal (in AString name, in nsIVariant value, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetGlobal(const nsAString& name, nsIVariant *value, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void removeByDomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void removeBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void removeGlobal (in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveGlobal(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void removeByDomain (in AString domain, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveByDomain(const nsAString& domain, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void removeBySubdomain (in AString domain, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveBySubdomain(const nsAString& domain, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void removeByName (in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveByName(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void removeAllDomains (in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAllDomains(nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void removeAllDomainsSince (in unsigned long long since, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAllDomainsSince(uint64_t since, nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void removeAllGlobals (in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAllGlobals(nsILoadContext *context, nsIContentPrefCallback2 *callback) = 0;

  /* void addObserverForName (in AString name, in nsIContentPrefObserver observer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddObserverForName(const nsAString& name, nsIContentPrefObserver *observer) = 0;

  /* void removeObserverForName (in AString name, in nsIContentPrefObserver observer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveObserverForName(const nsAString& name, nsIContentPrefObserver *observer) = 0;

  /* AString extractDomain (in AString str); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExtractDomain(const nsAString& str, nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentPrefService2, NS_ICONTENTPREFSERVICE2_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTPREFSERVICE2 \
  NS_IMETHOD GetByName(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD GetByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD GetBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD GetGlobal(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD GetCachedByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPref **_retval) override; \
  NS_IMETHOD GetCachedBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsTArray<RefPtr<nsIContentPref>>& _retval) override; \
  NS_IMETHOD GetCachedGlobal(const nsAString& name, nsILoadContext *context, nsIContentPref **_retval) override; \
  NS_IMETHOD Set(const nsAString& domain, const nsAString& name, nsIVariant *value, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD SetGlobal(const nsAString& name, nsIVariant *value, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD RemoveByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD RemoveBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD RemoveGlobal(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD RemoveByDomain(const nsAString& domain, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD RemoveBySubdomain(const nsAString& domain, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD RemoveByName(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD RemoveAllDomains(nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD RemoveAllDomainsSince(uint64_t since, nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD RemoveAllGlobals(nsILoadContext *context, nsIContentPrefCallback2 *callback) override; \
  NS_IMETHOD AddObserverForName(const nsAString& name, nsIContentPrefObserver *observer) override; \
  NS_IMETHOD RemoveObserverForName(const nsAString& name, nsIContentPrefObserver *observer) override; \
  NS_IMETHOD ExtractDomain(const nsAString& str, nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTPREFSERVICE2 \
  nsresult GetByName(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult GetByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult GetBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult GetGlobal(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult GetCachedByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPref **_retval); \
  nsresult GetCachedBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsTArray<RefPtr<nsIContentPref>>& _retval); \
  nsresult GetCachedGlobal(const nsAString& name, nsILoadContext *context, nsIContentPref **_retval); \
  nsresult Set(const nsAString& domain, const nsAString& name, nsIVariant *value, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult SetGlobal(const nsAString& name, nsIVariant *value, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult RemoveByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult RemoveBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult RemoveGlobal(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult RemoveByDomain(const nsAString& domain, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult RemoveBySubdomain(const nsAString& domain, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult RemoveByName(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult RemoveAllDomains(nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult RemoveAllDomainsSince(uint64_t since, nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult RemoveAllGlobals(nsILoadContext *context, nsIContentPrefCallback2 *callback); \
  nsresult AddObserverForName(const nsAString& name, nsIContentPrefObserver *observer); \
  nsresult RemoveObserverForName(const nsAString& name, nsIContentPrefObserver *observer); \
  nsresult ExtractDomain(const nsAString& str, nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTPREFSERVICE2(_to) \
  NS_IMETHOD GetByName(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to GetByName(name, context, callback); } \
  NS_IMETHOD GetByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to GetByDomainAndName(domain, name, context, callback); } \
  NS_IMETHOD GetBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to GetBySubdomainAndName(domain, name, context, callback); } \
  NS_IMETHOD GetGlobal(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to GetGlobal(name, context, callback); } \
  NS_IMETHOD GetCachedByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPref **_retval) override { return _to GetCachedByDomainAndName(domain, name, context, _retval); } \
  NS_IMETHOD GetCachedBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsTArray<RefPtr<nsIContentPref>>& _retval) override { return _to GetCachedBySubdomainAndName(domain, name, context, _retval); } \
  NS_IMETHOD GetCachedGlobal(const nsAString& name, nsILoadContext *context, nsIContentPref **_retval) override { return _to GetCachedGlobal(name, context, _retval); } \
  NS_IMETHOD Set(const nsAString& domain, const nsAString& name, nsIVariant *value, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to Set(domain, name, value, context, callback); } \
  NS_IMETHOD SetGlobal(const nsAString& name, nsIVariant *value, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to SetGlobal(name, value, context, callback); } \
  NS_IMETHOD RemoveByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to RemoveByDomainAndName(domain, name, context, callback); } \
  NS_IMETHOD RemoveBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to RemoveBySubdomainAndName(domain, name, context, callback); } \
  NS_IMETHOD RemoveGlobal(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to RemoveGlobal(name, context, callback); } \
  NS_IMETHOD RemoveByDomain(const nsAString& domain, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to RemoveByDomain(domain, context, callback); } \
  NS_IMETHOD RemoveBySubdomain(const nsAString& domain, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to RemoveBySubdomain(domain, context, callback); } \
  NS_IMETHOD RemoveByName(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to RemoveByName(name, context, callback); } \
  NS_IMETHOD RemoveAllDomains(nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to RemoveAllDomains(context, callback); } \
  NS_IMETHOD RemoveAllDomainsSince(uint64_t since, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to RemoveAllDomainsSince(since, context, callback); } \
  NS_IMETHOD RemoveAllGlobals(nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return _to RemoveAllGlobals(context, callback); } \
  NS_IMETHOD AddObserverForName(const nsAString& name, nsIContentPrefObserver *observer) override { return _to AddObserverForName(name, observer); } \
  NS_IMETHOD RemoveObserverForName(const nsAString& name, nsIContentPrefObserver *observer) override { return _to RemoveObserverForName(name, observer); } \
  NS_IMETHOD ExtractDomain(const nsAString& str, nsAString& _retval) override { return _to ExtractDomain(str, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTPREFSERVICE2(_to) \
  NS_IMETHOD GetByName(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetByName(name, context, callback); } \
  NS_IMETHOD GetByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetByDomainAndName(domain, name, context, callback); } \
  NS_IMETHOD GetBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBySubdomainAndName(domain, name, context, callback); } \
  NS_IMETHOD GetGlobal(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGlobal(name, context, callback); } \
  NS_IMETHOD GetCachedByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPref **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCachedByDomainAndName(domain, name, context, _retval); } \
  NS_IMETHOD GetCachedBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsTArray<RefPtr<nsIContentPref>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCachedBySubdomainAndName(domain, name, context, _retval); } \
  NS_IMETHOD GetCachedGlobal(const nsAString& name, nsILoadContext *context, nsIContentPref **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCachedGlobal(name, context, _retval); } \
  NS_IMETHOD Set(const nsAString& domain, const nsAString& name, nsIVariant *value, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Set(domain, name, value, context, callback); } \
  NS_IMETHOD SetGlobal(const nsAString& name, nsIVariant *value, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetGlobal(name, value, context, callback); } \
  NS_IMETHOD RemoveByDomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveByDomainAndName(domain, name, context, callback); } \
  NS_IMETHOD RemoveBySubdomainAndName(const nsAString& domain, const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveBySubdomainAndName(domain, name, context, callback); } \
  NS_IMETHOD RemoveGlobal(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveGlobal(name, context, callback); } \
  NS_IMETHOD RemoveByDomain(const nsAString& domain, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveByDomain(domain, context, callback); } \
  NS_IMETHOD RemoveBySubdomain(const nsAString& domain, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveBySubdomain(domain, context, callback); } \
  NS_IMETHOD RemoveByName(const nsAString& name, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveByName(name, context, callback); } \
  NS_IMETHOD RemoveAllDomains(nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAllDomains(context, callback); } \
  NS_IMETHOD RemoveAllDomainsSince(uint64_t since, nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAllDomainsSince(since, context, callback); } \
  NS_IMETHOD RemoveAllGlobals(nsILoadContext *context, nsIContentPrefCallback2 *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAllGlobals(context, callback); } \
  NS_IMETHOD AddObserverForName(const nsAString& name, nsIContentPrefObserver *observer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddObserverForName(name, observer); } \
  NS_IMETHOD RemoveObserverForName(const nsAString& name, nsIContentPrefObserver *observer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveObserverForName(name, observer); } \
  NS_IMETHOD ExtractDomain(const nsAString& str, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExtractDomain(str, _retval); } 


/* starting interface:    nsIContentPrefCallback2 */
#define NS_ICONTENTPREFCALLBACK2_IID_STR "1a12cf41-79e8-4d0f-9899-2f7b27c5d9a1"

#define NS_ICONTENTPREFCALLBACK2_IID \
  {0x1a12cf41, 0x79e8, 0x4d0f, \
    { 0x98, 0x99, 0x2f, 0x7b, 0x27, 0xc5, 0xd9, 0xa1 }}

class NS_NO_VTABLE nsIContentPrefCallback2 : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTPREFCALLBACK2_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentPrefCallback2;

  /* void handleResult (in nsIContentPref pref); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleResult(nsIContentPref *pref) = 0;

  /* void handleError (in nsresult error); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleError(nsresult error) = 0;

  /* void handleCompletion (in unsigned short reason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleCompletion(uint16_t reason) = 0;

  enum {
    COMPLETE_OK = 0U,
    COMPLETE_ERROR = 1U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentPrefCallback2, NS_ICONTENTPREFCALLBACK2_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTPREFCALLBACK2 \
  NS_IMETHOD HandleResult(nsIContentPref *pref) override; \
  NS_IMETHOD HandleError(nsresult error) override; \
  NS_IMETHOD HandleCompletion(uint16_t reason) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTPREFCALLBACK2 \
  nsresult HandleResult(nsIContentPref *pref); \
  nsresult HandleError(nsresult error); \
  nsresult HandleCompletion(uint16_t reason); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTPREFCALLBACK2(_to) \
  NS_IMETHOD HandleResult(nsIContentPref *pref) override { return _to HandleResult(pref); } \
  NS_IMETHOD HandleError(nsresult error) override { return _to HandleError(error); } \
  NS_IMETHOD HandleCompletion(uint16_t reason) override { return _to HandleCompletion(reason); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTPREFCALLBACK2(_to) \
  NS_IMETHOD HandleResult(nsIContentPref *pref) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleResult(pref); } \
  NS_IMETHOD HandleError(nsresult error) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleError(error); } \
  NS_IMETHOD HandleCompletion(uint16_t reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleCompletion(reason); } \


/* starting interface:    nsIContentPref */
#define NS_ICONTENTPREF_IID_STR "9f24948d-24b5-4b1b-b554-7dbd58c1d792"

#define NS_ICONTENTPREF_IID \
  {0x9f24948d, 0x24b5, 0x4b1b, \
    { 0xb5, 0x54, 0x7d, 0xbd, 0x58, 0xc1, 0xd7, 0x92 }}

class NS_NO_VTABLE nsIContentPref : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTPREF_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentPref;

  /* readonly attribute AString domain; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDomain(nsAString& aDomain) = 0;

  /* readonly attribute AString name; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetName(nsAString& aName) = 0;

  /* readonly attribute nsIVariant value; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetValue(nsIVariant **aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentPref, NS_ICONTENTPREF_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTPREF \
  NS_IMETHOD GetDomain(nsAString& aDomain) override; \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD GetValue(nsIVariant **aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTPREF \
  nsresult GetDomain(nsAString& aDomain); \
  nsresult GetName(nsAString& aName); \
  nsresult GetValue(nsIVariant **aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTPREF(_to) \
  NS_IMETHOD GetDomain(nsAString& aDomain) override { return _to GetDomain(aDomain); } \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetValue(nsIVariant **aValue) override { return _to GetValue(aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTPREF(_to) \
  NS_IMETHOD GetDomain(nsAString& aDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomain(aDomain); } \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetValue(nsIVariant **aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } 

// The contractID for the generic implementation built in to xpcom.
#define NS_CONTENT_PREF_SERVICE_CONTRACTID "@mozilla.org/content-pref/service;1"

#endif /* __gen_nsIContentPrefService2_h__ */
