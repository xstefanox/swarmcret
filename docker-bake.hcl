variable "BASE_IMAGE_NAME" {
  default = ""
}

target "default" {
  target = "default"
  tags = [
    "${BASE_IMAGE_NAME}-default",
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
    "${BASE_IMAGE_NAME}-alpine",
  ]
  cache-from = [
    "type=gha",
  ]
  cache-to = [
    "type=gha,mode=max",
  ]
}
