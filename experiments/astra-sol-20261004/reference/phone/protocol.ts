import type { AuctionRequest } from '../auction';

export interface AuctionPrice {
  schema: 'walt-auction-price-v1'; auction: AuctionRequest;
  worlds: number; inner_worlds: number; price: [number, string, string];
}
export interface PriceCall { auction_price: AuctionRequest; decl: number; worlds: number; budget_ms: number }
export interface MergeCall { auction_merge: AuctionRequest; worlds: number; receipts: AuctionPrice[] }
export interface WorkerMessage { id: number; checkpoint?: unknown; result?: unknown; error?: string }
