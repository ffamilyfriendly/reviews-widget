import { ENDPOINTS } from "./config";

interface I_ResultOk<T> {
  ok: true;
  data: T;
}

interface I_ResultErr<T> {
  ok: false;
  data: T;
}

type I_Result<T_OK, T_ERROR> = I_ResultOk<T_OK> | I_ResultErr<T_ERROR>;

export type Result<T, ERR = Error> = I_Result<T, ERR>;

export function Err(err: Error): Result<never> {
  return {
    ok: false,
    data: err,
  };
}

export function Ok<T>(data: T): Result<T> {
  return {
    ok: true,
    data,
  };
}

class ApiError extends Error {
  constructor(name, message) {
    super();
    this.message = message;
    this.name = name;
  }
}

function fetch_this<T>(path: string): Promise<Result<T>> {
  return new Promise((resolve) => {
    fetch(path)
      .then((data) => {
        if (data.status !== 200) {
          data
            .text()
            .then((text_error) => {
              let err;

              switch (data.status) {
                case 404:
                  err = new ApiError("Not Found", text_error);
                  break;
                case 429:
                  err = new ApiError("Ratelimits", text_error);
                  break;
                default:
                  err = new ApiError("Error", text_error);
              }

              resolve(Err(err));
            })
            .catch((e) => resolve(Err(e)));
        } else {
          data
            .json()
            .then((serialized_data) => {
              resolve(Ok(serialized_data));
            })
            .catch((e) => resolve(Err(e)));
        }
      })
      .catch((e) => resolve(Err(e)));
  });
}

export interface ScoreDistribution {
  key: number; // u8 in Rust is equivalent to number in TypeScript
  value: number; // u32 in Rust is equivalent to number in TypeScript
}

interface Score {
  average_score: number; // f32 in Rust is equivalent to number in TypeScript
  review_count: number; // u32 in Rust is equivalent to number in TypeScript
  score_distribution: ScoreDistribution[];
}

interface Author {
  username: string;
  avatar_url: string; // Alias in Rust: avatarUrl
}

export interface Review {
  content: string;
  score: number; // u8 in Rust is equivalent to number in TypeScript
  author: Author;
  timestamp: string;
}

export interface ReturnData {
  reviews: Review[];
  name: string;
  id: string;
  icon_url: string; // Alias in Rust: iconUrl
  review_stats: Score; // Alias in Rust: reviewStats
}

export function get_data(bot_id: string): Promise<Result<ReturnData>> {
  return fetch_this<ReturnData>(ENDPOINTS.GET_BASE_DATA(bot_id));
}
