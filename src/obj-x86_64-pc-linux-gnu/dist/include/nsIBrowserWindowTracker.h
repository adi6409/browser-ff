/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/modules/nsIBrowserWindowTracker.idl
 */

#ifndef __gen_nsIBrowserWindowTracker_h__
#define __gen_nsIBrowserWindowTracker_h__


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

/* starting interface:    nsIVisibleTab */
#define NS_IVISIBLETAB_IID_STR "f6190951-69d0-4ce5-b503-d2535d9de98c"

#define NS_IVISIBLETAB_IID \
  {0xf6190951, 0x69d0, 0x4ce5, \
    { 0xb5, 0x03, 0xd2, 0x53, 0x5d, 0x9d, 0xe9, 0x8c }}

class NS_NO_VTABLE nsIVisibleTab : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IVISIBLETAB_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIVisibleTab;

  /* attribute AString contentTitle; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentTitle(nsAString& aContentTitle) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetContentTitle(const nsAString& aContentTitle) = 0;

  /* attribute int64_t browserId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBrowserId(int64_t *aBrowserId) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetBrowserId(int64_t aBrowserId) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIVisibleTab, NS_IVISIBLETAB_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIVISIBLETAB \
  NS_IMETHOD GetContentTitle(nsAString& aContentTitle) override; \
  NS_IMETHOD SetContentTitle(const nsAString& aContentTitle) override; \
  NS_IMETHOD GetBrowserId(int64_t *aBrowserId) override; \
  NS_IMETHOD SetBrowserId(int64_t aBrowserId) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIVISIBLETAB \
  nsresult GetContentTitle(nsAString& aContentTitle); \
  nsresult SetContentTitle(const nsAString& aContentTitle); \
  nsresult GetBrowserId(int64_t *aBrowserId); \
  nsresult SetBrowserId(int64_t aBrowserId); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIVISIBLETAB(_to) \
  NS_IMETHOD GetContentTitle(nsAString& aContentTitle) override { return _to GetContentTitle(aContentTitle); } \
  NS_IMETHOD SetContentTitle(const nsAString& aContentTitle) override { return _to SetContentTitle(aContentTitle); } \
  NS_IMETHOD GetBrowserId(int64_t *aBrowserId) override { return _to GetBrowserId(aBrowserId); } \
  NS_IMETHOD SetBrowserId(int64_t aBrowserId) override { return _to SetBrowserId(aBrowserId); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIVISIBLETAB(_to) \
  NS_IMETHOD GetContentTitle(nsAString& aContentTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentTitle(aContentTitle); } \
  NS_IMETHOD SetContentTitle(const nsAString& aContentTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContentTitle(aContentTitle); } \
  NS_IMETHOD GetBrowserId(int64_t *aBrowserId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowserId(aBrowserId); } \
  NS_IMETHOD SetBrowserId(int64_t aBrowserId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBrowserId(aBrowserId); } 


/* starting interface:    nsIBrowserWindowTracker */
#define NS_IBROWSERWINDOWTRACKER_IID_STR "846ff245-ccbf-4c7a-807e-060f02927651"

#define NS_IBROWSERWINDOWTRACKER_IID \
  {0x846ff245, 0xccbf, 0x4c7a, \
    { 0x80, 0x7e, 0x06, 0x0f, 0x02, 0x92, 0x76, 0x51 }}

class NS_NO_VTABLE nsIBrowserWindowTracker : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBROWSERWINDOWTRACKER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBrowserWindowTracker;

  /* Array<nsIVisibleTab> getAllVisibleTabs (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAllVisibleTabs(nsTArray<RefPtr<nsIVisibleTab>>& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBrowserWindowTracker, NS_IBROWSERWINDOWTRACKER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBROWSERWINDOWTRACKER \
  NS_IMETHOD GetAllVisibleTabs(nsTArray<RefPtr<nsIVisibleTab>>& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBROWSERWINDOWTRACKER \
  nsresult GetAllVisibleTabs(nsTArray<RefPtr<nsIVisibleTab>>& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBROWSERWINDOWTRACKER(_to) \
  NS_IMETHOD GetAllVisibleTabs(nsTArray<RefPtr<nsIVisibleTab>>& _retval) override { return _to GetAllVisibleTabs(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBROWSERWINDOWTRACKER(_to) \
  NS_IMETHOD GetAllVisibleTabs(nsTArray<RefPtr<nsIVisibleTab>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllVisibleTabs(_retval); } 


#endif /* __gen_nsIBrowserWindowTracker_h__ */
