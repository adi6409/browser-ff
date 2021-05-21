/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIWebProgress.idl
 */

#ifndef __gen_nsIWebProgress_h__
#define __gen_nsIWebProgress_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindowProxy; /* forward declaration */

class nsIEventTarget; /* forward declaration */

class nsIWebProgressListener; /* forward declaration */


/* starting interface:    nsIWebProgress */
#define NS_IWEBPROGRESS_IID_STR "c4d64640-b332-4db6-a2a5-e08566000dc9"

#define NS_IWEBPROGRESS_IID \
  {0xc4d64640, 0xb332, 0x4db6, \
    { 0xa2, 0xa5, 0xe0, 0x85, 0x66, 0x00, 0x0d, 0xc9 }}

class NS_NO_VTABLE nsIWebProgress : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBPROGRESS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebProgress;

  enum {
    NOTIFY_STATE_REQUEST = 1U,
    NOTIFY_STATE_DOCUMENT = 2U,
    NOTIFY_STATE_NETWORK = 4U,
    NOTIFY_STATE_WINDOW = 8U,
    NOTIFY_STATE_ALL = 15U,
    NOTIFY_PROGRESS = 16U,
    NOTIFY_STATUS = 32U,
    NOTIFY_SECURITY = 64U,
    NOTIFY_LOCATION = 128U,
    NOTIFY_REFRESH = 256U,
    NOTIFY_CONTENT_BLOCKING = 512U,
    NOTIFY_ALL = 1023U
  };

  /* void addProgressListener (in nsIWebProgressListener aListener, in unsigned long aNotifyMask); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddProgressListener(nsIWebProgressListener *aListener, uint32_t aNotifyMask) = 0;

  /* void removeProgressListener (in nsIWebProgressListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveProgressListener(nsIWebProgressListener *aListener) = 0;

  /* readonly attribute mozIDOMWindowProxy DOMWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDOMWindow(mozIDOMWindowProxy **aDOMWindow) = 0;

  /* readonly attribute boolean isTopLevel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsTopLevel(bool *aIsTopLevel) = 0;

  /* readonly attribute boolean isLoadingDocument; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsLoadingDocument(bool *aIsLoadingDocument) = 0;

  /* readonly attribute unsigned long loadType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLoadType(uint32_t *aLoadType) = 0;

  /* attribute nsIEventTarget target; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTarget(nsIEventTarget **aTarget) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTarget(nsIEventTarget *aTarget) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebProgress, NS_IWEBPROGRESS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBPROGRESS \
  NS_IMETHOD AddProgressListener(nsIWebProgressListener *aListener, uint32_t aNotifyMask) override; \
  NS_IMETHOD RemoveProgressListener(nsIWebProgressListener *aListener) override; \
  NS_IMETHOD GetDOMWindow(mozIDOMWindowProxy **aDOMWindow) override; \
  NS_IMETHOD GetIsTopLevel(bool *aIsTopLevel) override; \
  NS_IMETHOD GetIsLoadingDocument(bool *aIsLoadingDocument) override; \
  NS_IMETHOD GetLoadType(uint32_t *aLoadType) override; \
  NS_IMETHOD GetTarget(nsIEventTarget **aTarget) override; \
  NS_IMETHOD SetTarget(nsIEventTarget *aTarget) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBPROGRESS \
  nsresult AddProgressListener(nsIWebProgressListener *aListener, uint32_t aNotifyMask); \
  nsresult RemoveProgressListener(nsIWebProgressListener *aListener); \
  nsresult GetDOMWindow(mozIDOMWindowProxy **aDOMWindow); \
  nsresult GetIsTopLevel(bool *aIsTopLevel); \
  nsresult GetIsLoadingDocument(bool *aIsLoadingDocument); \
  nsresult GetLoadType(uint32_t *aLoadType); \
  nsresult GetTarget(nsIEventTarget **aTarget); \
  nsresult SetTarget(nsIEventTarget *aTarget); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBPROGRESS(_to) \
  NS_IMETHOD AddProgressListener(nsIWebProgressListener *aListener, uint32_t aNotifyMask) override { return _to AddProgressListener(aListener, aNotifyMask); } \
  NS_IMETHOD RemoveProgressListener(nsIWebProgressListener *aListener) override { return _to RemoveProgressListener(aListener); } \
  NS_IMETHOD GetDOMWindow(mozIDOMWindowProxy **aDOMWindow) override { return _to GetDOMWindow(aDOMWindow); } \
  NS_IMETHOD GetIsTopLevel(bool *aIsTopLevel) override { return _to GetIsTopLevel(aIsTopLevel); } \
  NS_IMETHOD GetIsLoadingDocument(bool *aIsLoadingDocument) override { return _to GetIsLoadingDocument(aIsLoadingDocument); } \
  NS_IMETHOD GetLoadType(uint32_t *aLoadType) override { return _to GetLoadType(aLoadType); } \
  NS_IMETHOD GetTarget(nsIEventTarget **aTarget) override { return _to GetTarget(aTarget); } \
  NS_IMETHOD SetTarget(nsIEventTarget *aTarget) override { return _to SetTarget(aTarget); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBPROGRESS(_to) \
  NS_IMETHOD AddProgressListener(nsIWebProgressListener *aListener, uint32_t aNotifyMask) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddProgressListener(aListener, aNotifyMask); } \
  NS_IMETHOD RemoveProgressListener(nsIWebProgressListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveProgressListener(aListener); } \
  NS_IMETHOD GetDOMWindow(mozIDOMWindowProxy **aDOMWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDOMWindow(aDOMWindow); } \
  NS_IMETHOD GetIsTopLevel(bool *aIsTopLevel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsTopLevel(aIsTopLevel); } \
  NS_IMETHOD GetIsLoadingDocument(bool *aIsLoadingDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsLoadingDocument(aIsLoadingDocument); } \
  NS_IMETHOD GetLoadType(uint32_t *aLoadType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadType(aLoadType); } \
  NS_IMETHOD GetTarget(nsIEventTarget **aTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTarget(aTarget); } \
  NS_IMETHOD SetTarget(nsIEventTarget *aTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTarget(aTarget); } 


#endif /* __gen_nsIWebProgress_h__ */
