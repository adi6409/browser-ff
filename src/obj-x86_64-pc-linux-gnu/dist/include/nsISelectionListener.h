/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsISelectionListener.idl
 */

#ifndef __gen_nsISelectionListener_h__
#define __gen_nsISelectionListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Selection; /* webidl Selection */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsISelectionListener */
#define NS_ISELECTIONLISTENER_IID_STR "45686299-ae2b-46bc-9502-c56c35691ab9"

#define NS_ISELECTIONLISTENER_IID \
  {0x45686299, 0xae2b, 0x46bc, \
    { 0x95, 0x02, 0xc5, 0x6c, 0x35, 0x69, 0x1a, 0xb9 }}

class NS_NO_VTABLE nsISelectionListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISELECTIONLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISelectionListener;

  enum {
    NO_REASON = 0,
    DRAG_REASON = 1,
    MOUSEDOWN_REASON = 2,
    MOUSEUP_REASON = 4,
    KEYPRESS_REASON = 8,
    SELECTALL_REASON = 16,
    COLLAPSETOSTART_REASON = 32,
    COLLAPSETOEND_REASON = 64,
    IME_REASON = 128,
    JS_REASON = 256
  };

  /* [can_run_script] void notifySelectionChanged (in Document doc, in Selection sel, in short reason); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifySelectionChanged(mozilla::dom::Document *doc, mozilla::dom::Selection *sel, int16_t reason) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISelectionListener, NS_ISELECTIONLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISELECTIONLISTENER \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifySelectionChanged(mozilla::dom::Document *doc, mozilla::dom::Selection *sel, int16_t reason) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISELECTIONLISTENER \
  MOZ_CAN_RUN_SCRIPT nsresult NotifySelectionChanged(mozilla::dom::Document *doc, mozilla::dom::Selection *sel, int16_t reason); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISELECTIONLISTENER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifySelectionChanged(mozilla::dom::Document *doc, mozilla::dom::Selection *sel, int16_t reason) override { return _to NotifySelectionChanged(doc, sel, reason); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISELECTIONLISTENER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifySelectionChanged(mozilla::dom::Document *doc, mozilla::dom::Selection *sel, int16_t reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifySelectionChanged(doc, sel, reason); } 


#endif /* __gen_nsISelectionListener_h__ */
