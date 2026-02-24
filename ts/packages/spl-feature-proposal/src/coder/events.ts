import { Idl, Event, EventCoder } from "@trixter-osec/core";
import { IdlEvent } from "@trixter-osec/core/dist/cjs/idl";

export class SplFeatureProposalEventsCoder implements EventCoder {
  constructor(_idl: Idl) {}

  decode<E extends IdlEvent = IdlEvent, T = Record<string, string>>(
    _log: string
  ): Event<E, T> | null {
    throw new Error("SplFeatureProposal program does not have events");
  }
}
