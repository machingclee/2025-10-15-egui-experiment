use crate::prisma::PrismaClient;

pub fn get_db() -> &'static PrismaClient {
    crate::PRISMA_CLIENT.get().unwrap()
}
