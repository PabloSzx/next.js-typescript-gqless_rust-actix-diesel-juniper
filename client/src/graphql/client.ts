import { createUseQuery, createUseMutation } from "gqless-hooks";
import { schema, Query, Mutation } from "./generated";
import { IS_BROWSER } from "@src/utils/constants";

const endpoint = IS_BROWSER
  ? "/api/graphql"
  : "http://localhost:8000/api/graphql";

export const { useQuery, prepareQuery } = createUseQuery<Query>({
  schema,
  endpoint,
});

export const useMutation = createUseMutation<Mutation>({
  schema,
  endpoint,
});
