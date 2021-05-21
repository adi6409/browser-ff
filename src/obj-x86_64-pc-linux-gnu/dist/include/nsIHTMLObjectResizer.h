/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIHTMLObjectResizer.idl
 */

#ifndef __gen_nsIHTMLObjectResizer_h__
#define __gen_nsIHTMLObjectResizer_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIHTMLObjectResizer */
#define NS_IHTMLOBJECTRESIZER_IID_STR "8b396020-69d3-451f-80c1-1a96a7da25a9"

#define NS_IHTMLOBJECTRESIZER_IID \
  {0x8b396020, 0x69d3, 0x451f, \
    { 0x80, 0xc1, 0x1a, 0x96, 0xa7, 0xda, 0x25, 0xa9 }}

class nsIHTMLObjectResizer : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTMLOBJECTRESIZER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHTMLObjectResizer;

   typedef short EResizerLocation;
  enum {
    eTopLeft = 0,
    eTop = 1,
    eTopRight = 2,
    eLeft = 3,
    eRight = 4,
    eBottomLeft = 5,
    eBottom = 6,
    eBottomRight = 7
  };

  /* [can_run_script] attribute boolean objectResizingEnabled; */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetObjectResizingEnabled(bool *aObjectResizingEnabled) = 0;
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetObjectResizingEnabled(bool aObjectResizingEnabled) = 0;

  /* void hideResizers (); */
  NS_IMETHOD HideResizers(void) = 0;

  /* [can_run_script] void refreshResizers (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshResizers(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHTMLObjectResizer, NS_IHTMLOBJECTRESIZER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTMLOBJECTRESIZER \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetObjectResizingEnabled(bool *aObjectResizingEnabled) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetObjectResizingEnabled(bool aObjectResizingEnabled) override; \
  NS_IMETHOD HideResizers(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshResizers(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTMLOBJECTRESIZER \
  MOZ_CAN_RUN_SCRIPT nsresult GetObjectResizingEnabled(bool *aObjectResizingEnabled); \
  MOZ_CAN_RUN_SCRIPT nsresult SetObjectResizingEnabled(bool aObjectResizingEnabled); \
  nsresult HideResizers(void); \
  MOZ_CAN_RUN_SCRIPT nsresult RefreshResizers(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTMLOBJECTRESIZER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetObjectResizingEnabled(bool *aObjectResizingEnabled) override { return _to GetObjectResizingEnabled(aObjectResizingEnabled); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetObjectResizingEnabled(bool aObjectResizingEnabled) override { return _to SetObjectResizingEnabled(aObjectResizingEnabled); } \
  NS_IMETHOD HideResizers(void) override { return _to HideResizers(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshResizers(void) override { return _to RefreshResizers(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTMLOBJECTRESIZER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetObjectResizingEnabled(bool *aObjectResizingEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetObjectResizingEnabled(aObjectResizingEnabled); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetObjectResizingEnabled(bool aObjectResizingEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetObjectResizingEnabled(aObjectResizingEnabled); } \
  NS_IMETHOD HideResizers(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HideResizers(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshResizers(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RefreshResizers(); } 


#endif /* __gen_nsIHTMLObjectResizer_h__ */
