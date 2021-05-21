/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsISelectionDisplay.idl
 */

#ifndef __gen_nsISelectionDisplay_h__
#define __gen_nsISelectionDisplay_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsISelectionDisplay */
#define NS_ISELECTIONDISPLAY_IID_STR "0ddf9e1c-1dd2-11b2-a183-908a08aa75ae"

#define NS_ISELECTIONDISPLAY_IID \
  {0x0ddf9e1c, 0x1dd2, 0x11b2, \
    { 0xa1, 0x83, 0x90, 0x8a, 0x08, 0xaa, 0x75, 0xae }}

class NS_NO_VTABLE nsISelectionDisplay : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISELECTIONDISPLAY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISelectionDisplay;

  enum {
    DISPLAY_TEXT = 1,
    DISPLAY_IMAGES = 2,
    DISPLAY_FRAMES = 4,
    DISPLAY_ALL = 7
  };

  /* void setSelectionFlags (in short toggle); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSelectionFlags(int16_t toggle) = 0;

  /* short getSelectionFlags (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectionFlags(int16_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISelectionDisplay, NS_ISELECTIONDISPLAY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISELECTIONDISPLAY \
  NS_IMETHOD SetSelectionFlags(int16_t toggle) override; \
  NS_IMETHOD GetSelectionFlags(int16_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISELECTIONDISPLAY \
  nsresult SetSelectionFlags(int16_t toggle); \
  nsresult GetSelectionFlags(int16_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISELECTIONDISPLAY(_to) \
  NS_IMETHOD SetSelectionFlags(int16_t toggle) override { return _to SetSelectionFlags(toggle); } \
  NS_IMETHOD GetSelectionFlags(int16_t *_retval) override { return _to GetSelectionFlags(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISELECTIONDISPLAY(_to) \
  NS_IMETHOD SetSelectionFlags(int16_t toggle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSelectionFlags(toggle); } \
  NS_IMETHOD GetSelectionFlags(int16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectionFlags(_retval); } 


#endif /* __gen_nsISelectionDisplay_h__ */
