import Sidebar from "@/components/layout/Sidebar";
import Header from "@/components/layout/Header";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="antialiased flex h-screen overflow-hidden bg-slate-50">
        <Sidebar />
        <main className="flex-1 flex flex-col min-w-0">
          <Header />
          <div className="flex-1 overflow-auto p-6">
            {children}
          </div>
        </main>
      </body>
    </html>
  );
}
