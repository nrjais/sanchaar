import { Methods } from "@/models/methods";

export const methodColor = (method: Methods) => {
  switch (method) {
    case Methods.GET:
      return "#059669";
    case Methods.POST:
      return "#2563EB";
    case Methods.PUT:
      return "#DB2777";
    case Methods.DELETE:
      return "#DC2626";
    case Methods.PATCH:
      return "#F59E0B";
    case Methods.HEAD:
      return "#7C3AED";
    case Methods.OPTIONS:
      return "#14B8A6";
    default:
      return "#6B7280";
  }
};
