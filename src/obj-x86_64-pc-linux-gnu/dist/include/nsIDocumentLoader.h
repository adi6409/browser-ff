/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIDocumentLoader.idl
 */

#ifndef __gen_nsIDocumentLoader_h__
#define __gen_nsIDocumentLoader_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsILoadGroup; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIWebProgress; /* forward declaration */

class nsIRequest; /* forward declaration */


/* starting interface:    nsIDocumentLoader */
#define NS_IDOCUMENTLOADER_IID_STR "bbe961ee-59e9-42bb-be50-0331979bb79f"

#define NS_IDOCUMENTLOADER_IID \
  {0xbbe961ee, 0x59e9, 0x42bb, \
    { 0xbe, 0x50, 0x03, 0x31, 0x97, 0x9b, 0xb7, 0x9f }}

class NS_NO_VTABLE nsIDocumentLoader : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOCUMENTLOADER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDocumentLoader;

  /* void stop (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Stop(void) = 0;

  /* readonly attribute nsISupports container; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContainer(nsISupports **aContainer) = 0;

  /* readonly attribute nsILoadGroup loadGroup; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) = 0;

  /* readonly attribute nsIChannel documentChannel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDocumentChannel(nsIChannel **aDocumentChannel) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDocumentLoader, NS_IDOCUMENTLOADER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOCUMENTLOADER \
  NS_IMETHOD Stop(void) override; \
  NS_IMETHOD GetContainer(nsISupports **aContainer) override; \
  NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) override; \
  NS_IMETHOD GetDocumentChannel(nsIChannel **aDocumentChannel) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOCUMENTLOADER \
  nsresult Stop(void); \
  nsresult GetContainer(nsISupports **aContainer); \
  nsresult GetLoadGroup(nsILoadGroup **aLoadGroup); \
  nsresult GetDocumentChannel(nsIChannel **aDocumentChannel); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOCUMENTLOADER(_to) \
  NS_IMETHOD Stop(void) override { return _to Stop(); } \
  NS_IMETHOD GetContainer(nsISupports **aContainer) override { return _to GetContainer(aContainer); } \
  NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) override { return _to GetLoadGroup(aLoadGroup); } \
  NS_IMETHOD GetDocumentChannel(nsIChannel **aDocumentChannel) override { return _to GetDocumentChannel(aDocumentChannel); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOCUMENTLOADER(_to) \
  NS_IMETHOD Stop(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Stop(); } \
  NS_IMETHOD GetContainer(nsISupports **aContainer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContainer(aContainer); } \
  NS_IMETHOD GetLoadGroup(nsILoadGroup **aLoadGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadGroup(aLoadGroup); } \
  NS_IMETHOD GetDocumentChannel(nsIChannel **aDocumentChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocumentChannel(aDocumentChannel); } 


#endif /* __gen_nsIDocumentLoader_h__ */
