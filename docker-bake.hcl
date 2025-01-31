variable "SHA" {
  default = ""
}

variable "PLATFORM_TAG" {
  default = ""
}

target "default" {
  target = "default"
  tags = [
    "ghcr.io/xstefanox/swarmcret:${SHA}-${PLATFORM_TAG}-default",
  ]
  cache-from = [
    "type=gha",
  ]
  cache-to = [
    "type=gha,mode=max",
  ]
}

target "alpine" {
  target = "alpine"
  tags = [
    "ghcr.io/xstefanox/swarmcret:${SHA}-${PLATFORM_TAG}-alpine",
  ]
  cache-from = [
    "type=gha",
  ]
  cache-to = [
    "type=gha,mode=max",
  ]
}
