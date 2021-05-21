/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/composer/nsIEditingSession.idl
 */

#ifndef __gen_nsIEditingSession_h__
#define __gen_nsIEditingSession_h__


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
class mozIDOMWindowProxy; /* forward declaration */

class nsIEditor; /* forward declaration */

class mozIDOMWindowProxy;
namespace mozilla {
class HTMLEditor;
} // namespace mozilla

/* starting interface:    nsIEditingSession */
#define NS_IEDITINGSESSION_IID_STR "24f963d1-e6fc-43ea-a206-99ac5fcc5265"

#define NS_IEDITINGSESSION_IID \
  {0x24f963d1, 0xe6fc, 0x43ea, \
    { 0xa2, 0x06, 0x99, 0xac, 0x5f, 0xcc, 0x52, 0x65 }}

class nsIEditingSession : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEDITINGSESSION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEditingSession;

  enum {
    eEditorOK = 0,
    eEditorCreationInProgress = 1,
    eEditorErrorCantEditMimeType = 2,
    eEditorErrorFileNotFound = 3,
    eEditorErrorCantEditFramesets = 8,
    eEditorErrorUnknown = 9
  };

  /* readonly attribute unsigned long editorStatus; */
  NS_IMETHOD GetEditorStatus(uint32_t *aEditorStatus) = 0;

  /* [can_run_script] void makeWindowEditable (in mozIDOMWindowProxy window, in string aEditorType, in boolean doAfterUriLoad, in boolean aMakeWholeDocumentEditable, in boolean aInteractive); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MakeWindowEditable(mozIDOMWindowProxy *window, const char * aEditorType, bool doAfterUriLoad, bool aMakeWholeDocumentEditable, bool aInteractive) = 0;

  /* boolean windowIsEditable (in mozIDOMWindowProxy window); */
  NS_IMETHOD WindowIsEditable(mozIDOMWindowProxy *window, bool *_retval) = 0;

  /* nsIEditor getEditorForWindow (in mozIDOMWindowProxy window); */
  NS_IMETHOD GetEditorForWindow(mozIDOMWindowProxy *window, nsIEditor **_retval) = 0;

  /* [noscript] void tearDownEditorOnWindow (in mozIDOMWindowProxy window); */
  NS_IMETHOD TearDownEditorOnWindow(mozIDOMWindowProxy *window) = 0;

   /**
   * This method is implemented with nsIDocShell::GetHTMLEditor().  I.e.,
   * This method doesn't depend on nsEditingSession.  Therefore, even if
   * there were some implementation of nsIEditingSession interface, this
   * would be safe to use.
   */
  mozilla::HTMLEditor* GetHTMLEditorForWindow(mozIDOMWindowProxy* aWindow);
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEditingSession, NS_IEDITINGSESSION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEDITINGSESSION \
  NS_IMETHOD GetEditorStatus(uint32_t *aEditorStatus) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MakeWindowEditable(mozIDOMWindowProxy *window, const char * aEditorType, bool doAfterUriLoad, bool aMakeWholeDocumentEditable, bool aInteractive) override; \
  NS_IMETHOD WindowIsEditable(mozIDOMWindowProxy *window, bool *_retval) override; \
  NS_IMETHOD GetEditorForWindow(mozIDOMWindowProxy *window, nsIEditor **_retval) override; \
  NS_IMETHOD TearDownEditorOnWindow(mozIDOMWindowProxy *window) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEDITINGSESSION \
  nsresult GetEditorStatus(uint32_t *aEditorStatus); \
  MOZ_CAN_RUN_SCRIPT nsresult MakeWindowEditable(mozIDOMWindowProxy *window, const char * aEditorType, bool doAfterUriLoad, bool aMakeWholeDocumentEditable, bool aInteractive); \
  nsresult WindowIsEditable(mozIDOMWindowProxy *window, bool *_retval); \
  nsresult GetEditorForWindow(mozIDOMWindowProxy *window, nsIEditor **_retval); \
  nsresult TearDownEditorOnWindow(mozIDOMWindowProxy *window); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEDITINGSESSION(_to) \
  NS_IMETHOD GetEditorStatus(uint32_t *aEditorStatus) override { return _to GetEditorStatus(aEditorStatus); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MakeWindowEditable(mozIDOMWindowProxy *window, const char * aEditorType, bool doAfterUriLoad, bool aMakeWholeDocumentEditable, bool aInteractive) override { return _to MakeWindowEditable(window, aEditorType, doAfterUriLoad, aMakeWholeDocumentEditable, aInteractive); } \
  NS_IMETHOD WindowIsEditable(mozIDOMWindowProxy *window, bool *_retval) override { return _to WindowIsEditable(window, _retval); } \
  NS_IMETHOD GetEditorForWindow(mozIDOMWindowProxy *window, nsIEditor **_retval) override { return _to GetEditorForWindow(window, _retval); } \
  NS_IMETHOD TearDownEditorOnWindow(mozIDOMWindowProxy *window) override { return _to TearDownEditorOnWindow(window); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEDITINGSESSION(_to) \
  NS_IMETHOD GetEditorStatus(uint32_t *aEditorStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEditorStatus(aEditorStatus); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MakeWindowEditable(mozIDOMWindowProxy *window, const char * aEditorType, bool doAfterUriLoad, bool aMakeWholeDocumentEditable, bool aInteractive) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MakeWindowEditable(window, aEditorType, doAfterUriLoad, aMakeWholeDocumentEditable, aInteractive); } \
  NS_IMETHOD WindowIsEditable(mozIDOMWindowProxy *window, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WindowIsEditable(window, _retval); } \
  NS_IMETHOD GetEditorForWindow(mozIDOMWindowProxy *window, nsIEditor **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEditorForWindow(window, _retval); } \
  NS_IMETHOD TearDownEditorOnWindow(mozIDOMWindowProxy *window) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TearDownEditorOnWindow(window); } \


#endif /* __gen_nsIEditingSession_h__ */
