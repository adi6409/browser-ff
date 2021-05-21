/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/gmp/mozIGeckoMediaPluginChromeService.idl
 */

#ifndef __gen_mozIGeckoMediaPluginChromeService_h__
#define __gen_mozIGeckoMediaPluginChromeService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIFile_h__
#include "nsIFile.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozIGeckoMediaPluginChromeService */
#define MOZIGECKOMEDIAPLUGINCHROMESERVICE_IID_STR "32d35d21-181f-4630-8caa-a431e2ebad72"

#define MOZIGECKOMEDIAPLUGINCHROMESERVICE_IID \
  {0x32d35d21, 0x181f, 0x4630, \
    { 0x8c, 0xaa, 0xa4, 0x31, 0xe2, 0xeb, 0xad, 0x72 }}

class NS_NO_VTABLE mozIGeckoMediaPluginChromeService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIGECKOMEDIAPLUGINCHROMESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIGeckoMediaPluginChromeService;

  /* void addPluginDirectory (in AString directory); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddPluginDirectory(const nsAString& directory) = 0;

  /* void removePluginDirectory (in AString directory); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemovePluginDirectory(const nsAString& directory) = 0;

  /* void removeAndDeletePluginDirectory (in AString directory, [optional] in bool defer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAndDeletePluginDirectory(const nsAString& directory, bool defer) = 0;

  /* void forgetThisSite (in AString site, in AString aPattern); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ForgetThisSite(const nsAString& site, const nsAString& aPattern) = 0;

  /* bool isPersistentStorageAllowed (in ACString nodeId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsPersistentStorageAllowed(const nsACString& nodeId, bool *_retval) = 0;

  /* nsIFile getStorageDir (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStorageDir(nsIFile **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIGeckoMediaPluginChromeService, MOZIGECKOMEDIAPLUGINCHROMESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIGECKOMEDIAPLUGINCHROMESERVICE \
  NS_IMETHOD AddPluginDirectory(const nsAString& directory) override; \
  NS_IMETHOD RemovePluginDirectory(const nsAString& directory) override; \
  NS_IMETHOD RemoveAndDeletePluginDirectory(const nsAString& directory, bool defer) override; \
  NS_IMETHOD ForgetThisSite(const nsAString& site, const nsAString& aPattern) override; \
  NS_IMETHOD IsPersistentStorageAllowed(const nsACString& nodeId, bool *_retval) override; \
  NS_IMETHOD GetStorageDir(nsIFile **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIGECKOMEDIAPLUGINCHROMESERVICE \
  nsresult AddPluginDirectory(const nsAString& directory); \
  nsresult RemovePluginDirectory(const nsAString& directory); \
  nsresult RemoveAndDeletePluginDirectory(const nsAString& directory, bool defer); \
  nsresult ForgetThisSite(const nsAString& site, const nsAString& aPattern); \
  nsresult IsPersistentStorageAllowed(const nsACString& nodeId, bool *_retval); \
  nsresult GetStorageDir(nsIFile **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIGECKOMEDIAPLUGINCHROMESERVICE(_to) \
  NS_IMETHOD AddPluginDirectory(const nsAString& directory) override { return _to AddPluginDirectory(directory); } \
  NS_IMETHOD RemovePluginDirectory(const nsAString& directory) override { return _to RemovePluginDirectory(directory); } \
  NS_IMETHOD RemoveAndDeletePluginDirectory(const nsAString& directory, bool defer) override { return _to RemoveAndDeletePluginDirectory(directory, defer); } \
  NS_IMETHOD ForgetThisSite(const nsAString& site, const nsAString& aPattern) override { return _to ForgetThisSite(site, aPattern); } \
  NS_IMETHOD IsPersistentStorageAllowed(const nsACString& nodeId, bool *_retval) override { return _to IsPersistentStorageAllowed(nodeId, _retval); } \
  NS_IMETHOD GetStorageDir(nsIFile **_retval) override { return _to GetStorageDir(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIGECKOMEDIAPLUGINCHROMESERVICE(_to) \
  NS_IMETHOD AddPluginDirectory(const nsAString& directory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddPluginDirectory(directory); } \
  NS_IMETHOD RemovePluginDirectory(const nsAString& directory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemovePluginDirectory(directory); } \
  NS_IMETHOD RemoveAndDeletePluginDirectory(const nsAString& directory, bool defer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAndDeletePluginDirectory(directory, defer); } \
  NS_IMETHOD ForgetThisSite(const nsAString& site, const nsAString& aPattern) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForgetThisSite(site, aPattern); } \
  NS_IMETHOD IsPersistentStorageAllowed(const nsACString& nodeId, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsPersistentStorageAllowed(nodeId, _retval); } \
  NS_IMETHOD GetStorageDir(nsIFile **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStorageDir(_retval); } 


#endif /* __gen_mozIGeckoMediaPluginChromeService_h__ */
