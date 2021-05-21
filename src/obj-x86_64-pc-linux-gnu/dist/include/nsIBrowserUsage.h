/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIBrowserUsage.idl
 */

#ifndef __gen_nsIBrowserUsage_h__
#define __gen_nsIBrowserUsage_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIBrowserUsage */
#define NS_IBROWSERUSAGE_IID_STR "2703b5ed-a41f-42be-8764-b795eb67ed25"

#define NS_IBROWSERUSAGE_IID \
  {0x2703b5ed, 0xa41f, 0x42be, \
    { 0x87, 0x64, 0xb7, 0x95, 0xeb, 0x67, 0xed, 0x25 }}

class NS_NO_VTABLE nsIBrowserUsage : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBROWSERUSAGE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBrowserUsage;

  /* uint32_t getUniqueDomainsVisitedInPast24Hours (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUniqueDomainsVisitedInPast24Hours(uint32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBrowserUsage, NS_IBROWSERUSAGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBROWSERUSAGE \
  NS_IMETHOD GetUniqueDomainsVisitedInPast24Hours(uint32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBROWSERUSAGE \
  nsresult GetUniqueDomainsVisitedInPast24Hours(uint32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBROWSERUSAGE(_to) \
  NS_IMETHOD GetUniqueDomainsVisitedInPast24Hours(uint32_t *_retval) override { return _to GetUniqueDomainsVisitedInPast24Hours(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBROWSERUSAGE(_to) \
  NS_IMETHOD GetUniqueDomainsVisitedInPast24Hours(uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUniqueDomainsVisitedInPast24Hours(_retval); } 


#endif /* __gen_nsIBrowserUsage_h__ */
