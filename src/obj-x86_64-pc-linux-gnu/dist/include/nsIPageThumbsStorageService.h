/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/thumbnails/nsIPageThumbsStorageService.idl
 */

#ifndef __gen_nsIPageThumbsStorageService_h__
#define __gen_nsIPageThumbsStorageService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPageThumbsStorageService */
#define NS_IPAGETHUMBSSTORAGESERVICE_IID_STR "97943eec-0e48-49ef-b7b7-cf4aa0109bb6"

#define NS_IPAGETHUMBSSTORAGESERVICE_IID \
  {0x97943eec, 0x0e48, 0x49ef, \
    { 0xb7, 0xb7, 0xcf, 0x4a, 0xa0, 0x10, 0x9b, 0xb6 }}

class NS_NO_VTABLE nsIPageThumbsStorageService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAGETHUMBSSTORAGESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPageThumbsStorageService;

  /* AString getLeafNameForURL (in AString aURL); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLeafNameForURL(const nsAString& aURL, nsAString& _retval) = 0;

  /* readonly attribute AString path; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPath(nsAString& aPath) = 0;

  /* AString getFilePathForURL (in AString aURL); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFilePathForURL(const nsAString& aURL, nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPageThumbsStorageService, NS_IPAGETHUMBSSTORAGESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAGETHUMBSSTORAGESERVICE \
  NS_IMETHOD GetLeafNameForURL(const nsAString& aURL, nsAString& _retval) override; \
  NS_IMETHOD GetPath(nsAString& aPath) override; \
  NS_IMETHOD GetFilePathForURL(const nsAString& aURL, nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAGETHUMBSSTORAGESERVICE \
  nsresult GetLeafNameForURL(const nsAString& aURL, nsAString& _retval); \
  nsresult GetPath(nsAString& aPath); \
  nsresult GetFilePathForURL(const nsAString& aURL, nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAGETHUMBSSTORAGESERVICE(_to) \
  NS_IMETHOD GetLeafNameForURL(const nsAString& aURL, nsAString& _retval) override { return _to GetLeafNameForURL(aURL, _retval); } \
  NS_IMETHOD GetPath(nsAString& aPath) override { return _to GetPath(aPath); } \
  NS_IMETHOD GetFilePathForURL(const nsAString& aURL, nsAString& _retval) override { return _to GetFilePathForURL(aURL, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAGETHUMBSSTORAGESERVICE(_to) \
  NS_IMETHOD GetLeafNameForURL(const nsAString& aURL, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLeafNameForURL(aURL, _retval); } \
  NS_IMETHOD GetPath(nsAString& aPath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPath(aPath); } \
  NS_IMETHOD GetFilePathForURL(const nsAString& aURL, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFilePathForURL(aURL, _retval); } 


#endif /* __gen_nsIPageThumbsStorageService_h__ */
