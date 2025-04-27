import { z } from "zod";

export const ytDltSchema = z.object({
    title: z.string().min(1).max(50),
    description: z.string().min(0).max(50).optional(),
    startTime: z.number().min(0),
    endTime: z
        .number({
            required_error: "End time is required",
        })
        .min(0),
    createdAt: z.date().optional(),
});

export type ytDltFormSchema = typeof ytDltSchema;
