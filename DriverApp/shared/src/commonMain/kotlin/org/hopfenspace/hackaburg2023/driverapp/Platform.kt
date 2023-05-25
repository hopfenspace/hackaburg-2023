package org.hopfenspace.hackaburg2023.driverapp

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform