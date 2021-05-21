/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsITextInputProcessorCallback.idl
 */

#ifndef __gen_nsITextInputProcessorCallback_h__
#define __gen_nsITextInputProcessorCallback_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsITextInputProcessor; /* forward declaration */


/* starting interface:    nsITextInputProcessorNotification */
#define NS_ITEXTINPUTPROCESSORNOTIFICATION_IID_STR "c0ce1add-82bb-45ab-b99a-42cfba7fd5d7"

#define NS_ITEXTINPUTPROCESSORNOTIFICATION_IID \
  {0xc0ce1add, 0x82bb, 0x45ab, \
    { 0xb9, 0x9a, 0x42, 0xcf, 0xba, 0x7f, 0xd5, 0xd7 }}

class NS_NO_VTABLE nsITextInputProcessorNotification : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITEXTINPUTPROCESSORNOTIFICATION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITextInputProcessorNotification;

  /* readonly attribute ACString type; */
  NS_IMETHOD GetType(nsACString& aType) = 0;

  /* readonly attribute unsigned long offset; */
  NS_IMETHOD GetOffset(uint32_t *aOffset) = 0;

  /* readonly attribute AString text; */
  NS_IMETHOD GetText(nsAString& aText) = 0;

  /* readonly attribute boolean collapsed; */
  NS_IMETHOD GetCollapsed(bool *aCollapsed) = 0;

  /* readonly attribute uint32_t length; */
  NS_IMETHOD GetLength(uint32_t *aLength) = 0;

  /* readonly attribute boolean reversed; */
  NS_IMETHOD GetReversed(bool *aReversed) = 0;

  /* readonly attribute ACString writingMode; */
  NS_IMETHOD GetWritingMode(nsACString& aWritingMode) = 0;

  /* readonly attribute boolean causedByComposition; */
  NS_IMETHOD GetCausedByComposition(bool *aCausedByComposition) = 0;

  /* readonly attribute boolean causedBySelectionEvent; */
  NS_IMETHOD GetCausedBySelectionEvent(bool *aCausedBySelectionEvent) = 0;

  /* readonly attribute boolean occurredDuringComposition; */
  NS_IMETHOD GetOccurredDuringComposition(bool *aOccurredDuringComposition) = 0;

  /* readonly attribute unsigned long removedLength; */
  NS_IMETHOD GetRemovedLength(uint32_t *aRemovedLength) = 0;

  /* readonly attribute unsigned long addedLength; */
  NS_IMETHOD GetAddedLength(uint32_t *aAddedLength) = 0;

  /* readonly attribute boolean causedOnlyByComposition; */
  NS_IMETHOD GetCausedOnlyByComposition(bool *aCausedOnlyByComposition) = 0;

  /* readonly attribute boolean includingChangesDuringComposition; */
  NS_IMETHOD GetIncludingChangesDuringComposition(bool *aIncludingChangesDuringComposition) = 0;

  /* readonly attribute boolean includingChangesWithoutComposition; */
  NS_IMETHOD GetIncludingChangesWithoutComposition(bool *aIncludingChangesWithoutComposition) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITextInputProcessorNotification, NS_ITEXTINPUTPROCESSORNOTIFICATION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITEXTINPUTPROCESSORNOTIFICATION \
  NS_IMETHOD GetType(nsACString& aType) override; \
  NS_IMETHOD GetOffset(uint32_t *aOffset) override; \
  NS_IMETHOD GetText(nsAString& aText) override; \
  NS_IMETHOD GetCollapsed(bool *aCollapsed) override; \
  NS_IMETHOD GetLength(uint32_t *aLength) override; \
  NS_IMETHOD GetReversed(bool *aReversed) override; \
  NS_IMETHOD GetWritingMode(nsACString& aWritingMode) override; \
  NS_IMETHOD GetCausedByComposition(bool *aCausedByComposition) override; \
  NS_IMETHOD GetCausedBySelectionEvent(bool *aCausedBySelectionEvent) override; \
  NS_IMETHOD GetOccurredDuringComposition(bool *aOccurredDuringComposition) override; \
  NS_IMETHOD GetRemovedLength(uint32_t *aRemovedLength) override; \
  NS_IMETHOD GetAddedLength(uint32_t *aAddedLength) override; \
  NS_IMETHOD GetCausedOnlyByComposition(bool *aCausedOnlyByComposition) override; \
  NS_IMETHOD GetIncludingChangesDuringComposition(bool *aIncludingChangesDuringComposition) override; \
  NS_IMETHOD GetIncludingChangesWithoutComposition(bool *aIncludingChangesWithoutComposition) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITEXTINPUTPROCESSORNOTIFICATION \
  nsresult GetType(nsACString& aType); \
  nsresult GetOffset(uint32_t *aOffset); \
  nsresult GetText(nsAString& aText); \
  nsresult GetCollapsed(bool *aCollapsed); \
  nsresult GetLength(uint32_t *aLength); \
  nsresult GetReversed(bool *aReversed); \
  nsresult GetWritingMode(nsACString& aWritingMode); \
  nsresult GetCausedByComposition(bool *aCausedByComposition); \
  nsresult GetCausedBySelectionEvent(bool *aCausedBySelectionEvent); \
  nsresult GetOccurredDuringComposition(bool *aOccurredDuringComposition); \
  nsresult GetRemovedLength(uint32_t *aRemovedLength); \
  nsresult GetAddedLength(uint32_t *aAddedLength); \
  nsresult GetCausedOnlyByComposition(bool *aCausedOnlyByComposition); \
  nsresult GetIncludingChangesDuringComposition(bool *aIncludingChangesDuringComposition); \
  nsresult GetIncludingChangesWithoutComposition(bool *aIncludingChangesWithoutComposition); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITEXTINPUTPROCESSORNOTIFICATION(_to) \
  NS_IMETHOD GetType(nsACString& aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetOffset(uint32_t *aOffset) override { return _to GetOffset(aOffset); } \
  NS_IMETHOD GetText(nsAString& aText) override { return _to GetText(aText); } \
  NS_IMETHOD GetCollapsed(bool *aCollapsed) override { return _to GetCollapsed(aCollapsed); } \
  NS_IMETHOD GetLength(uint32_t *aLength) override { return _to GetLength(aLength); } \
  NS_IMETHOD GetReversed(bool *aReversed) override { return _to GetReversed(aReversed); } \
  NS_IMETHOD GetWritingMode(nsACString& aWritingMode) override { return _to GetWritingMode(aWritingMode); } \
  NS_IMETHOD GetCausedByComposition(bool *aCausedByComposition) override { return _to GetCausedByComposition(aCausedByComposition); } \
  NS_IMETHOD GetCausedBySelectionEvent(bool *aCausedBySelectionEvent) override { return _to GetCausedBySelectionEvent(aCausedBySelectionEvent); } \
  NS_IMETHOD GetOccurredDuringComposition(bool *aOccurredDuringComposition) override { return _to GetOccurredDuringComposition(aOccurredDuringComposition); } \
  NS_IMETHOD GetRemovedLength(uint32_t *aRemovedLength) override { return _to GetRemovedLength(aRemovedLength); } \
  NS_IMETHOD GetAddedLength(uint32_t *aAddedLength) override { return _to GetAddedLength(aAddedLength); } \
  NS_IMETHOD GetCausedOnlyByComposition(bool *aCausedOnlyByComposition) override { return _to GetCausedOnlyByComposition(aCausedOnlyByComposition); } \
  NS_IMETHOD GetIncludingChangesDuringComposition(bool *aIncludingChangesDuringComposition) override { return _to GetIncludingChangesDuringComposition(aIncludingChangesDuringComposition); } \
  NS_IMETHOD GetIncludingChangesWithoutComposition(bool *aIncludingChangesWithoutComposition) override { return _to GetIncludingChangesWithoutComposition(aIncludingChangesWithoutComposition); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITEXTINPUTPROCESSORNOTIFICATION(_to) \
  NS_IMETHOD GetType(nsACString& aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetOffset(uint32_t *aOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOffset(aOffset); } \
  NS_IMETHOD GetText(nsAString& aText) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetText(aText); } \
  NS_IMETHOD GetCollapsed(bool *aCollapsed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCollapsed(aCollapsed); } \
  NS_IMETHOD GetLength(uint32_t *aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLength(aLength); } \
  NS_IMETHOD GetReversed(bool *aReversed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReversed(aReversed); } \
  NS_IMETHOD GetWritingMode(nsACString& aWritingMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWritingMode(aWritingMode); } \
  NS_IMETHOD GetCausedByComposition(bool *aCausedByComposition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCausedByComposition(aCausedByComposition); } \
  NS_IMETHOD GetCausedBySelectionEvent(bool *aCausedBySelectionEvent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCausedBySelectionEvent(aCausedBySelectionEvent); } \
  NS_IMETHOD GetOccurredDuringComposition(bool *aOccurredDuringComposition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOccurredDuringComposition(aOccurredDuringComposition); } \
  NS_IMETHOD GetRemovedLength(uint32_t *aRemovedLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRemovedLength(aRemovedLength); } \
  NS_IMETHOD GetAddedLength(uint32_t *aAddedLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAddedLength(aAddedLength); } \
  NS_IMETHOD GetCausedOnlyByComposition(bool *aCausedOnlyByComposition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCausedOnlyByComposition(aCausedOnlyByComposition); } \
  NS_IMETHOD GetIncludingChangesDuringComposition(bool *aIncludingChangesDuringComposition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIncludingChangesDuringComposition(aIncludingChangesDuringComposition); } \
  NS_IMETHOD GetIncludingChangesWithoutComposition(bool *aIncludingChangesWithoutComposition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIncludingChangesWithoutComposition(aIncludingChangesWithoutComposition); } 


/* starting interface:    nsITextInputProcessorCallback */
#define NS_ITEXTINPUTPROCESSORCALLBACK_IID_STR "23d5f242-adb5-46f1-8766-90d1bf0383df"

#define NS_ITEXTINPUTPROCESSORCALLBACK_IID \
  {0x23d5f242, 0xadb5, 0x46f1, \
    { 0x87, 0x66, 0x90, 0xd1, 0xbf, 0x03, 0x83, 0xdf }}

class NS_NO_VTABLE nsITextInputProcessorCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITEXTINPUTPROCESSORCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITextInputProcessorCallback;

  /* boolean onNotify (in nsITextInputProcessor aTextInputProcessor, in nsITextInputProcessorNotification aNotification); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnNotify(nsITextInputProcessor *aTextInputProcessor, nsITextInputProcessorNotification *aNotification, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITextInputProcessorCallback, NS_ITEXTINPUTPROCESSORCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITEXTINPUTPROCESSORCALLBACK \
  NS_IMETHOD OnNotify(nsITextInputProcessor *aTextInputProcessor, nsITextInputProcessorNotification *aNotification, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITEXTINPUTPROCESSORCALLBACK \
  nsresult OnNotify(nsITextInputProcessor *aTextInputProcessor, nsITextInputProcessorNotification *aNotification, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITEXTINPUTPROCESSORCALLBACK(_to) \
  NS_IMETHOD OnNotify(nsITextInputProcessor *aTextInputProcessor, nsITextInputProcessorNotification *aNotification, bool *_retval) override { return _to OnNotify(aTextInputProcessor, aNotification, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITEXTINPUTPROCESSORCALLBACK(_to) \
  NS_IMETHOD OnNotify(nsITextInputProcessor *aTextInputProcessor, nsITextInputProcessorNotification *aNotification, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnNotify(aTextInputProcessor, aNotification, _retval); } 


#endif /* __gen_nsITextInputProcessorCallback_h__ */
