/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/xre/nsIXREDirProvider.idl
 */

#ifndef __gen_nsIXREDirProvider_h__
#define __gen_nsIXREDirProvider_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */


/* starting interface:    nsIXREDirProvider */
#define NS_IXREDIRPROVIDER_IID_STR "f6ee3c0a-5119-47fc-b1a7-ace9e1111fff"

#define NS_IXREDIRPROVIDER_IID \
  {0xf6ee3c0a, 0x5119, 0x47fc, \
    { 0xb1, 0xa7, 0xac, 0xe9, 0xe1, 0x11, 0x1f, 0xff }}

class NS_NO_VTABLE nsIXREDirProvider : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXREDIRPROVIDER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXREDirProvider;

  /* void setUserDataDirectory (in nsIFile aFile, in boolean aLocal); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetUserDataDirectory(nsIFile *aFile, bool aLocal) = 0;

  /* AString getInstallHash (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInstallHash(nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXREDirProvider, NS_IXREDIRPROVIDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXREDIRPROVIDER \
  NS_IMETHOD SetUserDataDirectory(nsIFile *aFile, bool aLocal) override; \
  NS_IMETHOD GetInstallHash(nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXREDIRPROVIDER \
  nsresult SetUserDataDirectory(nsIFile *aFile, bool aLocal); \
  nsresult GetInstallHash(nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXREDIRPROVIDER(_to) \
  NS_IMETHOD SetUserDataDirectory(nsIFile *aFile, bool aLocal) override { return _to SetUserDataDirectory(aFile, aLocal); } \
  NS_IMETHOD GetInstallHash(nsAString& _retval) override { return _to GetInstallHash(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXREDIRPROVIDER(_to) \
  NS_IMETHOD SetUserDataDirectory(nsIFile *aFile, bool aLocal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUserDataDirectory(aFile, aLocal); } \
  NS_IMETHOD GetInstallHash(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInstallHash(_retval); } 


#endif /* __gen_nsIXREDirProvider_h__ */
