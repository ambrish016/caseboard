// The board asks this every few minutes. When the answer changes, a new deploy
// is live and the board shows "New version ready".
export const dynamic = "force-dynamic";

export function GET() {
  const version =
    process.env.VERCEL_GIT_COMMIT_SHA ||
    process.env.VERCEL_DEPLOYMENT_ID ||
    "local";
  return Response.json(
    { version },
    { headers: { "Cache-Control": "no-store" } }
  );
}
