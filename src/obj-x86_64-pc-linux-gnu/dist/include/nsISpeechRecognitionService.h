/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/webspeech/recognition/nsISpeechRecognitionService.idl
 */

#ifndef __gen_nsISpeechRecognitionService_h__
#define __gen_nsISpeechRecognitionService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "mozilla/WeakPtr.h"
namespace mozilla {
class AudioSegment;
namespace dom {
class SpeechRecognition;
class SpeechRecognitionResultList;
class SpeechGrammarList;
class SpeechGrammar;
}
}

/* starting interface:    nsISpeechGrammarCompilationCallback */
#define NS_ISPEECHGRAMMARCOMPILATIONCALLBACK_IID_STR "6fcb6ee8-a6db-49ba-9f06-355d7ee18ea7"

#define NS_ISPEECHGRAMMARCOMPILATIONCALLBACK_IID \
  {0x6fcb6ee8, 0xa6db, 0x49ba, \
    { 0x9f, 0x06, 0x35, 0x5d, 0x7e, 0xe1, 0x8e, 0xa7 }}

class NS_NO_VTABLE nsISpeechGrammarCompilationCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISPEECHGRAMMARCOMPILATIONCALLBACK_IID)

  /* void grammarCompilationEnd (in SpeechGrammarPtr grammarObject, in boolean success); */
  NS_IMETHOD GrammarCompilationEnd(mozilla::dom::SpeechGrammar * grammarObject, bool success) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISpeechGrammarCompilationCallback, NS_ISPEECHGRAMMARCOMPILATIONCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISPEECHGRAMMARCOMPILATIONCALLBACK \
  NS_IMETHOD GrammarCompilationEnd(mozilla::dom::SpeechGrammar * grammarObject, bool success) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISPEECHGRAMMARCOMPILATIONCALLBACK \
  nsresult GrammarCompilationEnd(mozilla::dom::SpeechGrammar * grammarObject, bool success); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISPEECHGRAMMARCOMPILATIONCALLBACK(_to) \
  NS_IMETHOD GrammarCompilationEnd(mozilla::dom::SpeechGrammar * grammarObject, bool success) override { return _to GrammarCompilationEnd(grammarObject, success); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISPEECHGRAMMARCOMPILATIONCALLBACK(_to) \
  NS_IMETHOD GrammarCompilationEnd(mozilla::dom::SpeechGrammar * grammarObject, bool success) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GrammarCompilationEnd(grammarObject, success); } 


/* starting interface:    nsISpeechRecognitionService */
#define NS_ISPEECHRECOGNITIONSERVICE_IID_STR "8e97f287-f322-44e8-8888-8344fa408ef8"

#define NS_ISPEECHRECOGNITIONSERVICE_IID \
  {0x8e97f287, 0xf322, 0x44e8, \
    { 0x88, 0x88, 0x83, 0x44, 0xfa, 0x40, 0x8e, 0xf8 }}

class NS_NO_VTABLE nsISpeechRecognitionService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISPEECHRECOGNITIONSERVICE_IID)

  /* void initialize (in SpeechRecognitionWeakPtr aSpeechRecognition); */
  NS_IMETHOD Initialize(mozilla::WeakPtr<mozilla::dom::SpeechRecognition> aSpeechRecognition) = 0;

  /* void processAudioSegment (in AudioSegmentPtr aAudioSegment, in long aSampleRate); */
  NS_IMETHOD ProcessAudioSegment(mozilla::AudioSegment * aAudioSegment, int32_t aSampleRate) = 0;

  /* void validateAndSetGrammarList (in SpeechGrammarPtr aSpeechGrammar, in nsISpeechGrammarCompilationCallback aCallback); */
  NS_IMETHOD ValidateAndSetGrammarList(mozilla::dom::SpeechGrammar * aSpeechGrammar, nsISpeechGrammarCompilationCallback *aCallback) = 0;

  /* void soundEnd (); */
  NS_IMETHOD SoundEnd(void) = 0;

  /* void abort (); */
  NS_IMETHOD Abort(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISpeechRecognitionService, NS_ISPEECHRECOGNITIONSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISPEECHRECOGNITIONSERVICE \
  NS_IMETHOD Initialize(mozilla::WeakPtr<mozilla::dom::SpeechRecognition> aSpeechRecognition) override; \
  NS_IMETHOD ProcessAudioSegment(mozilla::AudioSegment * aAudioSegment, int32_t aSampleRate) override; \
  NS_IMETHOD ValidateAndSetGrammarList(mozilla::dom::SpeechGrammar * aSpeechGrammar, nsISpeechGrammarCompilationCallback *aCallback) override; \
  NS_IMETHOD SoundEnd(void) override; \
  NS_IMETHOD Abort(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISPEECHRECOGNITIONSERVICE \
  nsresult Initialize(mozilla::WeakPtr<mozilla::dom::SpeechRecognition> aSpeechRecognition); \
  nsresult ProcessAudioSegment(mozilla::AudioSegment * aAudioSegment, int32_t aSampleRate); \
  nsresult ValidateAndSetGrammarList(mozilla::dom::SpeechGrammar * aSpeechGrammar, nsISpeechGrammarCompilationCallback *aCallback); \
  nsresult SoundEnd(void); \
  nsresult Abort(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISPEECHRECOGNITIONSERVICE(_to) \
  NS_IMETHOD Initialize(mozilla::WeakPtr<mozilla::dom::SpeechRecognition> aSpeechRecognition) override { return _to Initialize(aSpeechRecognition); } \
  NS_IMETHOD ProcessAudioSegment(mozilla::AudioSegment * aAudioSegment, int32_t aSampleRate) override { return _to ProcessAudioSegment(aAudioSegment, aSampleRate); } \
  NS_IMETHOD ValidateAndSetGrammarList(mozilla::dom::SpeechGrammar * aSpeechGrammar, nsISpeechGrammarCompilationCallback *aCallback) override { return _to ValidateAndSetGrammarList(aSpeechGrammar, aCallback); } \
  NS_IMETHOD SoundEnd(void) override { return _to SoundEnd(); } \
  NS_IMETHOD Abort(void) override { return _to Abort(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISPEECHRECOGNITIONSERVICE(_to) \
  NS_IMETHOD Initialize(mozilla::WeakPtr<mozilla::dom::SpeechRecognition> aSpeechRecognition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Initialize(aSpeechRecognition); } \
  NS_IMETHOD ProcessAudioSegment(mozilla::AudioSegment * aAudioSegment, int32_t aSampleRate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProcessAudioSegment(aAudioSegment, aSampleRate); } \
  NS_IMETHOD ValidateAndSetGrammarList(mozilla::dom::SpeechGrammar * aSpeechGrammar, nsISpeechGrammarCompilationCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ValidateAndSetGrammarList(aSpeechGrammar, aCallback); } \
  NS_IMETHOD SoundEnd(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SoundEnd(); } \
  NS_IMETHOD Abort(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Abort(); } 

#define NS_SPEECH_RECOGNITION_SERVICE_CONTRACTID_PREFIX "@mozilla.org/webspeech/service;1?name="

#endif /* __gen_nsISpeechRecognitionService_h__ */
