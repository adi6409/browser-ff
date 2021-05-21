/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/modules/nsIRegion.idl
 */

#ifndef __gen_nsIRegion_h__
#define __gen_nsIRegion_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIRegion */
#define NS_IREGION_IID_STR "21e6d094-e016-41a4-80cd-76d2e20871aa"

#define NS_IREGION_IID \
  {0x21e6d094, 0xe016, 0x41a4, \
    { 0x80, 0xcd, 0x76, 0xd2, 0xe2, 0x08, 0x71, 0xaa }}

class NS_NO_VTABLE nsIRegion : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREGION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRegion;

  /* readonly attribute AString current; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrent(nsAString& aCurrent) = 0;

  /* readonly attribute AString home; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHome(nsAString& aHome) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRegion, NS_IREGION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREGION \
  NS_IMETHOD GetCurrent(nsAString& aCurrent) override; \
  NS_IMETHOD GetHome(nsAString& aHome) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREGION \
  nsresult GetCurrent(nsAString& aCurrent); \
  nsresult GetHome(nsAString& aHome); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREGION(_to) \
  NS_IMETHOD GetCurrent(nsAString& aCurrent) override { return _to GetCurrent(aCurrent); } \
  NS_IMETHOD GetHome(nsAString& aHome) override { return _to GetHome(aHome); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREGION(_to) \
  NS_IMETHOD GetCurrent(nsAString& aCurrent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrent(aCurrent); } \
  NS_IMETHOD GetHome(nsAString& aHome) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHome(aHome); } 

/**
 * The observer topic to listen to for Region notifications.
 */
#define REGION_TOPIC "browser-region-updated"

#endif /* __gen_nsIRegion_h__ */
