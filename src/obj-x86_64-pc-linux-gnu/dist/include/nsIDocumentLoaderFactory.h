/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIDocumentLoaderFactory.idl
 */

#ifndef __gen_nsIDocumentLoaderFactory_h__
#define __gen_nsIDocumentLoaderFactory_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIContentViewer; /* forward declaration */

class nsIStreamListener; /* forward declaration */

class nsIDocShell; /* forward declaration */

class nsILoadGroup; /* forward declaration */

class nsIPrincipal; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDocumentLoaderFactory */
#define NS_IDOCUMENTLOADERFACTORY_IID_STR "e795239e-9d3c-47c4-b063-9e600fb3b287"

#define NS_IDOCUMENTLOADERFACTORY_IID \
  {0xe795239e, 0x9d3c, 0x47c4, \
    { 0xb0, 0x63, 0x9e, 0x60, 0x0f, 0xb3, 0xb2, 0x87 }}

class NS_NO_VTABLE nsIDocumentLoaderFactory : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOCUMENTLOADERFACTORY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDocumentLoaderFactory;

  /* nsIContentViewer createInstance (in string aCommand, in nsIChannel aChannel, in nsILoadGroup aLoadGroup, in ACString aContentType, in nsIDocShell aContainer, in nsISupports aExtraInfo, out nsIStreamListener aDocListenerResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateInstance(const char * aCommand, nsIChannel *aChannel, nsILoadGroup *aLoadGroup, const nsACString& aContentType, nsIDocShell *aContainer, nsISupports *aExtraInfo, nsIStreamListener **aDocListenerResult, nsIContentViewer **_retval) = 0;

  /* nsIContentViewer createInstanceForDocument (in nsISupports aContainer, in Document aDocument, in string aCommand); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateInstanceForDocument(nsISupports *aContainer, mozilla::dom::Document *aDocument, const char * aCommand, nsIContentViewer **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDocumentLoaderFactory, NS_IDOCUMENTLOADERFACTORY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOCUMENTLOADERFACTORY \
  NS_IMETHOD CreateInstance(const char * aCommand, nsIChannel *aChannel, nsILoadGroup *aLoadGroup, const nsACString& aContentType, nsIDocShell *aContainer, nsISupports *aExtraInfo, nsIStreamListener **aDocListenerResult, nsIContentViewer **_retval) override; \
  NS_IMETHOD CreateInstanceForDocument(nsISupports *aContainer, mozilla::dom::Document *aDocument, const char * aCommand, nsIContentViewer **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOCUMENTLOADERFACTORY \
  nsresult CreateInstance(const char * aCommand, nsIChannel *aChannel, nsILoadGroup *aLoadGroup, const nsACString& aContentType, nsIDocShell *aContainer, nsISupports *aExtraInfo, nsIStreamListener **aDocListenerResult, nsIContentViewer **_retval); \
  nsresult CreateInstanceForDocument(nsISupports *aContainer, mozilla::dom::Document *aDocument, const char * aCommand, nsIContentViewer **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOCUMENTLOADERFACTORY(_to) \
  NS_IMETHOD CreateInstance(const char * aCommand, nsIChannel *aChannel, nsILoadGroup *aLoadGroup, const nsACString& aContentType, nsIDocShell *aContainer, nsISupports *aExtraInfo, nsIStreamListener **aDocListenerResult, nsIContentViewer **_retval) override { return _to CreateInstance(aCommand, aChannel, aLoadGroup, aContentType, aContainer, aExtraInfo, aDocListenerResult, _retval); } \
  NS_IMETHOD CreateInstanceForDocument(nsISupports *aContainer, mozilla::dom::Document *aDocument, const char * aCommand, nsIContentViewer **_retval) override { return _to CreateInstanceForDocument(aContainer, aDocument, aCommand, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOCUMENTLOADERFACTORY(_to) \
  NS_IMETHOD CreateInstance(const char * aCommand, nsIChannel *aChannel, nsILoadGroup *aLoadGroup, const nsACString& aContentType, nsIDocShell *aContainer, nsISupports *aExtraInfo, nsIStreamListener **aDocListenerResult, nsIContentViewer **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateInstance(aCommand, aChannel, aLoadGroup, aContentType, aContainer, aExtraInfo, aDocListenerResult, _retval); } \
  NS_IMETHOD CreateInstanceForDocument(nsISupports *aContainer, mozilla::dom::Document *aDocument, const char * aCommand, nsIContentViewer **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateInstanceForDocument(aContainer, aDocument, aCommand, _retval); } 


#endif /* __gen_nsIDocumentLoaderFactory_h__ */
