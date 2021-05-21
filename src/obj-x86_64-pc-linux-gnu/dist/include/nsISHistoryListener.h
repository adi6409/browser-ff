/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/shistory/nsISHistoryListener.idl
 */

#ifndef __gen_nsISHistoryListener_h__
#define __gen_nsISHistoryListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */


/* starting interface:    nsISHistoryListener */
#define NS_ISHISTORYLISTENER_IID_STR "125c0833-746a-400e-9b89-d2d18545c08a"

#define NS_ISHISTORYLISTENER_IID \
  {0x125c0833, 0x746a, 0x400e, \
    { 0x9b, 0x89, 0xd2, 0xd1, 0x85, 0x45, 0xc0, 0x8a }}

class NS_NO_VTABLE nsISHistoryListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISHISTORYLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISHistoryListener;

  /* void OnHistoryNewEntry (in nsIURI aNewURI, in long aOldIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnHistoryNewEntry(nsIURI *aNewURI, int32_t aOldIndex) = 0;

  /* boolean OnHistoryReload (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnHistoryReload(bool *_retval) = 0;

  /* void OnHistoryGotoIndex (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnHistoryGotoIndex(void) = 0;

  /* void OnHistoryPurge (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnHistoryPurge(void) = 0;

  /* void OnHistoryReplaceEntry (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnHistoryReplaceEntry(void) = 0;

  /* void OnContentViewerEvicted (in unsigned long aNumEvicted); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnContentViewerEvicted(uint32_t aNumEvicted) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISHistoryListener, NS_ISHISTORYLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISHISTORYLISTENER \
  NS_IMETHOD OnHistoryNewEntry(nsIURI *aNewURI, int32_t aOldIndex) override; \
  NS_IMETHOD OnHistoryReload(bool *_retval) override; \
  NS_IMETHOD OnHistoryGotoIndex(void) override; \
  NS_IMETHOD OnHistoryPurge(void) override; \
  NS_IMETHOD OnHistoryReplaceEntry(void) override; \
  NS_IMETHOD OnContentViewerEvicted(uint32_t aNumEvicted) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISHISTORYLISTENER \
  nsresult OnHistoryNewEntry(nsIURI *aNewURI, int32_t aOldIndex); \
  nsresult OnHistoryReload(bool *_retval); \
  nsresult OnHistoryGotoIndex(void); \
  nsresult OnHistoryPurge(void); \
  nsresult OnHistoryReplaceEntry(void); \
  nsresult OnContentViewerEvicted(uint32_t aNumEvicted); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISHISTORYLISTENER(_to) \
  NS_IMETHOD OnHistoryNewEntry(nsIURI *aNewURI, int32_t aOldIndex) override { return _to OnHistoryNewEntry(aNewURI, aOldIndex); } \
  NS_IMETHOD OnHistoryReload(bool *_retval) override { return _to OnHistoryReload(_retval); } \
  NS_IMETHOD OnHistoryGotoIndex(void) override { return _to OnHistoryGotoIndex(); } \
  NS_IMETHOD OnHistoryPurge(void) override { return _to OnHistoryPurge(); } \
  NS_IMETHOD OnHistoryReplaceEntry(void) override { return _to OnHistoryReplaceEntry(); } \
  NS_IMETHOD OnContentViewerEvicted(uint32_t aNumEvicted) override { return _to OnContentViewerEvicted(aNumEvicted); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISHISTORYLISTENER(_to) \
  NS_IMETHOD OnHistoryNewEntry(nsIURI *aNewURI, int32_t aOldIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnHistoryNewEntry(aNewURI, aOldIndex); } \
  NS_IMETHOD OnHistoryReload(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnHistoryReload(_retval); } \
  NS_IMETHOD OnHistoryGotoIndex(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnHistoryGotoIndex(); } \
  NS_IMETHOD OnHistoryPurge(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnHistoryPurge(); } \
  NS_IMETHOD OnHistoryReplaceEntry(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnHistoryReplaceEntry(); } \
  NS_IMETHOD OnContentViewerEvicted(uint32_t aNumEvicted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnContentViewerEvicted(aNumEvicted); } 


#endif /* __gen_nsISHistoryListener_h__ */
