fn main() {
    // Generate Prisma Client Rust whenever schema.prisma changes
    println!("cargo:rerun-if-changed=prisma/schema.prisma");

    // This generates the Prisma client
    prisma_client_rust::cli::run();
}
