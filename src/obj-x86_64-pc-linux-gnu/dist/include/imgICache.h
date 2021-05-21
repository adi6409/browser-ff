/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgICache.idl
 */

#ifndef __gen_imgICache_h__
#define __gen_imgICache_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class imgIRequest; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIProperties; /* forward declaration */

class nsIURI; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

namespace mozilla {
class OriginAttributes;
} // mozilla namespace

/* starting interface:    imgICache */
#define IMGICACHE_IID_STR "bfdf23ff-378e-402e-8a6c-840f0c82b6c3"

#define IMGICACHE_IID \
  {0xbfdf23ff, 0x378e, 0x402e, \
    { 0x8a, 0x6c, 0x84, 0x0f, 0x0c, 0x82, 0xb6, 0xc3 }}

class NS_NO_VTABLE imgICache : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(IMGICACHE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = imgICache;

  /* void clearCache (in boolean chrome); */
  NS_IMETHOD ClearCache(bool chrome) = 0;

  /* [noscript] void removeEntry (in nsIURI uri, [optional] in Document doc); */
  NS_IMETHOD RemoveEntry(nsIURI *uri, mozilla::dom::Document *doc) = 0;

  /* void removeEntriesFromPrincipal (in nsIPrincipal aPrincipal); */
  NS_IMETHOD RemoveEntriesFromPrincipal(nsIPrincipal *aPrincipal) = 0;

  /* [must_use] nsIProperties findEntryProperties (in nsIURI uri, [optional] in Document doc); */
  [[nodiscard]] NS_IMETHOD FindEntryProperties(nsIURI *uri, mozilla::dom::Document *doc, nsIProperties **_retval) = 0;

  /* void respectPrivacyNotifications (); */
  NS_IMETHOD RespectPrivacyNotifications(void) = 0;

  /* [noscript,notxpcom] void clearCacheForControlledDocument (in Document doc); */
  NS_IMETHOD_(void) ClearCacheForControlledDocument(mozilla::dom::Document *doc) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(imgICache, IMGICACHE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_IMGICACHE \
  NS_IMETHOD ClearCache(bool chrome) override; \
  NS_IMETHOD RemoveEntry(nsIURI *uri, mozilla::dom::Document *doc) override; \
  NS_IMETHOD RemoveEntriesFromPrincipal(nsIPrincipal *aPrincipal) override; \
  [[nodiscard]] NS_IMETHOD FindEntryProperties(nsIURI *uri, mozilla::dom::Document *doc, nsIProperties **_retval) override; \
  NS_IMETHOD RespectPrivacyNotifications(void) override; \
  NS_IMETHOD_(void) ClearCacheForControlledDocument(mozilla::dom::Document *doc) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_IMGICACHE \
  nsresult ClearCache(bool chrome); \
  nsresult RemoveEntry(nsIURI *uri, mozilla::dom::Document *doc); \
  nsresult RemoveEntriesFromPrincipal(nsIPrincipal *aPrincipal); \
  [[nodiscard]] nsresult FindEntryProperties(nsIURI *uri, mozilla::dom::Document *doc, nsIProperties **_retval); \
  nsresult RespectPrivacyNotifications(void); \
  nsresult_(void) ClearCacheForControlledDocument(mozilla::dom::Document *doc); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_IMGICACHE(_to) \
  NS_IMETHOD ClearCache(bool chrome) override { return _to ClearCache(chrome); } \
  NS_IMETHOD RemoveEntry(nsIURI *uri, mozilla::dom::Document *doc) override { return _to RemoveEntry(uri, doc); } \
  NS_IMETHOD RemoveEntriesFromPrincipal(nsIPrincipal *aPrincipal) override { return _to RemoveEntriesFromPrincipal(aPrincipal); } \
  [[nodiscard]] NS_IMETHOD FindEntryProperties(nsIURI *uri, mozilla::dom::Document *doc, nsIProperties **_retval) override { return _to FindEntryProperties(uri, doc, _retval); } \
  NS_IMETHOD RespectPrivacyNotifications(void) override { return _to RespectPrivacyNotifications(); } \
  NS_IMETHOD_(void) ClearCacheForControlledDocument(mozilla::dom::Document *doc) override { return _to ClearCacheForControlledDocument(doc); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_IMGICACHE(_to) \
  NS_IMETHOD ClearCache(bool chrome) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearCache(chrome); } \
  NS_IMETHOD RemoveEntry(nsIURI *uri, mozilla::dom::Document *doc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveEntry(uri, doc); } \
  NS_IMETHOD RemoveEntriesFromPrincipal(nsIPrincipal *aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveEntriesFromPrincipal(aPrincipal); } \
  [[nodiscard]] NS_IMETHOD FindEntryProperties(nsIURI *uri, mozilla::dom::Document *doc, nsIProperties **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FindEntryProperties(uri, doc, _retval); } \
  NS_IMETHOD RespectPrivacyNotifications(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RespectPrivacyNotifications(); } \
  NS_IMETHOD_(void) ClearCacheForControlledDocument(mozilla::dom::Document *doc) override; 


#endif /* __gen_imgICache_h__ */
